#undef _GLIBCXX_DEBUG                			// Disable run-time bound checking, etc
#pragma GCC optimize("Ofast,inline")			// Ofast = O3,fast-math,allow-store-data-races,no-protect-parens
#pragma GCC target("bmi,bmi2,lzcnt,popcnt")		// Bit manipulation
#pragma GCC target("movbe")                     // Byte swap


#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/pgmspace.h>
#include <avr/eeprom.h>

#include "lcd.h"
#include "math_utils.h"

#include <math.h>


#define BACK_BTN    (1<<PD5)
#define FORWARD_BTN (1<<PD6)
#define RIGHT_BTN   (1<<PD7)
#define LEFT_BTN    (1<<PB0)

#define BAK_INT_BTN (1<<PCINT21)
#define FWD_INT_BTN (1<<PCINT22)
#define RT_INT_BTN  (1<<PCINT23)
#define LT_INT_BTN  (1<<PCINT0)


// constexpr int16_t iminus = toFixed<8>(-1.0f/0.64836082745);
// constexpr int16_t fminus = toFloat<8>(-395);
// (int32_t(-1) << 8) / int32_t(0.64836082745);

// constexpr int16_t tng = toFixed<8>(tan(ra));
// constexpr int16_t iminus = (int32_t(-1) << 8) / int32_t(tng);

// constexpr float at = -1/0.64836082745;



template<size_t dp>
constexpr int16_t toFixed(float d) {
    return int16_t(d * float(1<<dp) + (d >= 0 ? 0.5 : -0.5));
}

template<size_t dp>
constexpr float toFloat(int16_t d) {
    return float(d) / float(1<<dp);
}


#undef M_PI
#undef M_PI_2  // I confused myself with my variable names so many times...

constexpr int16_t RT      = toFixed<8>(0.05);
constexpr int16_t DR      = toFixed<8>(0.0174533);
constexpr int16_t M_PI    = toFixed<8>(3.14159265358979323846);     
constexpr int16_t M_PI_2  = toFixed<8>(1.57079632679489661923);
constexpr int16_t M_2PI   = toFixed<8>(2*3.14159265358979323846);
constexpr int16_t M_3PI_2 = toFixed<8>(3*1.57079632679489661923);


struct Player {
    int16_t x = toFixed<8>(70.0f); 
    int16_t y = toFixed<8>(13.0f); 
    
    const static uint8_t size  = 2; 
    const static uint8_t speed = 1;

    const float fangle = 90.0f;
    int16_t angle = toFixed<8>(fangle); 
    int16_t dx = toFixed<8>(cos(fangle)*5.0f); 
    int16_t dy = toFixed<8>(sin(fangle)*5.0f);
}; Player player;

struct MapConfig {
    const static uint8_t mapX = 12; 
    const static uint8_t mapY = 8;
    const static uint8_t cell = 8;

    uint8_t map[mapX*mapY] = {
        1,1,1,1,1,1,1,1,1,1,1,1,
        1,0,0,0,0,1,0,0,0,0,1,1,
        1,0,0,0,0,0,0,0,0,1,1,1,
        1,0,0,0,1,0,0,1,0,0,1,1,
        1,0,0,0,0,0,0,0,0,1,1,1,
        1,1,1,1,1,1,1,1,1,1,1,1,
    };
}; constexpr MapConfig mapConfig;



void drawMap() {
    cli();

    int mx = mapConfig.mapX;
    int my = mapConfig.mapY;

    for(int y = 0; y < my; y++) {
        for(int x = 0; x < mx; x++) {
            int c = mapConfig.cell;
            int xo = x * c;
            int yo = y * c;

            if(mapConfig.map[y * mx + x] == 1) {LCD.rect(xo,yo, c,c);}
        }
    }

    sei();
}

// constexpr float sdist(float x1, float y1, float x2, float y2) {return std::sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1));}
int16_t sdist(int16_t x1, int16_t y1, int16_t x2, int16_t y2) {return sqrt(((x2-x1)*(x2-x1))  +  ((y2-y1)*(y2-y1)));}

void castRays() {
    uint8_t dof = 0; 
    int16_t rayX = 0, rayY = 0, rayAngle = 0;
    constexpr uint8_t fov = 30;

    constexpr uint8_t cell = mapConfig.cell, mapW = mapConfig.mapX, mapH = mapConfig.mapY;
    uint8_t mapX = 0, mapY = 0;
    int16_t mapPos = 0;
    int16_t offsetX = 0, offsetY = 0;

    int16_t playerX = player.x;
    int16_t playerY = player.y;
    int16_t playerAngle = player.angle;

    constexpr int16_t rounding = 9;     // Magic number  -  comes from int32_t(0.00015f * float(1<<16));


    // constexpr int16_t a = toFixed<8>(5.6);
    // constexpr int16_t b = toFixed<8>(2.7);

    // constexpr int16_t c = a + b;
    // constexpr int16_t d = a - b;

    // constexpr int16_t e = (int32_t(a) * int32_t(b)) >> 8;
    // constexpr int16_t fe = (int32_t(a) << 8) / int32_t(b);


    // Ray angle in radians
    rayAngle = playerAngle;
    if(rayAngle < 0)     rayAngle += M_2PI;
    if(rayAngle > M_2PI) rayAngle -= M_2PI;
    
    cli();
    for(uint8_t r = 0; r < fov; r++) {
        // Horizontal line
        int16_t distH = 1000000;
        int16_t horX = playerX, horY = playerY;

        dof = 0;
        int16_t tng = toFixed<8>(tan(toFloat<8>(rayAngle)));
        int16_t aTan = (int32_t(-1) << 8) / int32_t(tng);

        if(rayAngle > M_PI) {
            int16_t pd = (int32_t(playerY) << 8) / int32_t(cell);  // pd = playerY / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayY = pm - rounding;
            rayX = (playerY-rayY) * aTan + playerX;

            offsetY = -cell;
            offsetX = -offsetY * aTan;
        }
        if(rayAngle < M_PI) {
            int16_t pd = (int32_t(playerY) << 8) / int32_t(cell);  // pd = playerY / cell
            int16_t pm = (int32_t(pm) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayY = pm + cell;
            rayX = (playerY-rayY) * aTan + playerX;

            offsetY = cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle == 0 || rayAngle == M_PI) {
            rayX = playerX;
            rayY = playerY;
            dof = 8;
        }


        while(dof < 8) {
            mapX = (int32_t(rayX) << 8) / int32_t(cell);  // rayX/cell
            mapY = (int32_t(rayY) << 8) / int32_t(cell);  // rayY/cell
            mapPos = mapY * mapW + mapX;

            // Wall hit
            if(mapPos > 0  &&  mapPos < mapW*mapH  &&  mapConfig.map[mapPos] > 0) {
                horX = rayX;
                horY = rayY;
                distH = sdist(playerX, playerY, horX, horY);

                dof = 8;
            } else {
                rayX += offsetX;
                rayY += offsetY;
                dof += 1;
            }
        }


        // Vertical line
        int16_t distV = 1000000;
        int16_t vertX = playerX, vertY = playerY;

        dof = 0;
        int16_t nTan = -toFixed<8>(tan(toFloat<8>(rayAngle)));


        if(rayAngle > M_PI_2 && rayAngle < M_3PI_2) {
            int16_t pd = (int32_t(playerX) << 8) / int32_t(cell);  // pd = playerX / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayX = pm - rounding;
            rayY = (playerX-rayX) * nTan + playerY;

            offsetX = -cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle < M_PI_2 || rayAngle > M_3PI_2) {
            int16_t pd = (int32_t(playerX) << 8) / int32_t(cell);  // pd = playerX / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayX = pm + cell;
            rayY = (playerX-rayX) * nTan + playerY;

            offsetX =  cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle == 0 || rayAngle == M_PI) {
            rayX = playerX;
            rayY = playerY;
            dof = 8;
        }


        while(dof < 8) {
            mapX = (int32_t(rayX) << 8) / int32_t(cell);  // rayX/cell
            mapY = (int32_t(rayY) << 8) / int32_t(cell);  // rayY/cell
            mapPos = mapY * mapW + mapX;

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapConfig.map[mapPos] > 0)) {
                vertX = rayX;
                vertY = rayY;
                distV = sdist(playerX, playerY, vertX, vertY);

                dof = 8;
            } else {
                rayX += offsetX;
                rayY += offsetY;
                dof += 1;
            }
        }

        if(distV < distH) {
            rayX = vertX;
            rayY = vertY;
        }
        if(distH < distV) {
            rayX = horX;
            rayY = horY;
        }

        float lpX = toFloat<8>(playerX);
        float lpY = toFloat<8>(playerY);
        float rpX = toFloat<8>(rayX);
        float rpY = toFloat<8>(rayY);

        LCD.line(lpX, lpY, rpX, rpY);
        
        rayAngle += DR*3;
        if(rayAngle < 0)    rayAngle += M_2PI;
        if(rayAngle > M_2PI) rayAngle -= M_2PI;
    }

    sei();
}

void drawPlayer() {
    cli();
    LCD.rect(toFloat<8>(player.x), toFloat<8>(player.y),  player.size, player.size);
    castRays();
    sei();
}


int main(){
    // -- Navigation buttons --
    DDRD  &= ~(RIGHT_BTN | FORWARD_BTN | BACK_BTN);         // PD7/PCINT23, PD6/PCINT22 and PD5/PCINT21 as input    
    PORTD |=  (RIGHT_BTN | FORWARD_BTN | BACK_BTN);         // PD7/PCINT23, PD6/PCINT22 and PD5/PCINT21 internal pull-up resistor

    DDRB  &= ~LEFT_BTN;                                     // PB0/PCINT0 as input
    PORTB |=  LEFT_BTN;                                     // PB0/PCINT0 internal pull-up resistor

    PCICR  |= ((1<<PCIE0) | (1<<PCIE2));                    // Set PCIE0 and PCIE2 to enable PCMSK0 and PCMSK2 scan
    PCIFR  |= ((1<<PCIF0) | (1<<PCIF2));                    // Set PCIF0 and PCIF2 to enable interrupt flasg mask
    PCMSK0 |= LT_INT_BTN;                                   // Set PCINT0 to trigger an interrupt on state change 
    PCMSK2 |= (FWD_INT_BTN | RT_INT_BTN | BAK_INT_BTN);     // Set PCINT21, PCINT22 and PCINT23 to trigger an interrupt on state change
    // -! Navigation buttons !-


    LCD.init();

    sei();
    while(1) {
        drawMap();
        drawPlayer();

        LCD.render();
        LCD.clear();
    } 
    return 0;
}


// -- Navigation buttons --
ISR(PCINT0_vect) {
    if(!(PINB & LEFT_BTN)) {
        // Low state on LEFT_BTN

        player.angle -= RT;
        if(player.angle < 0) player.angle += M_2PI;
        player.dx = toFixed<8>(cos(toFloat<8>(player.angle))*5);
        player.dy = toFixed<8>(sin(toFloat<8>(player.angle))*5);

        // player.angle -= 0.05;
        // if(player.angle < 0) player.angle += 2*M_PI;
        // player.dx = cos(player.angle)*5;
        // player.dy = sin(player.angle)*5;
    } 
}

ISR(PCINT2_vect) {
    if(!(PIND & BACK_BTN)) {
        // Low state on BACK_BTN
        // player.y += player.speed;

        player.x -= player.dx;
        player.y -= player.dy;
    } 

    if(!(PIND & FORWARD_BTN)) {
        // Low state on FORWARD_BTN
        // player.y -= player.speed;

        player.x += player.dx;
        player.y += player.dy;
    } 

    if(!(PIND & RIGHT_BTN)) {
        // Low state on RIGHT_BTN
        // player.x += player.speed;

        player.angle += RT;
        if(player.angle > M_2PI) player.angle -= M_2PI;
        player.dx = toFixed<8>(cos(toFloat<8>(player.angle))*5);
        player.dy = toFixed<8>(sin(toFloat<8>(player.angle))*5);

        // player.angle += 0.05;
        // if(player.angle > 2*M_PI) player.angle -= 2*M_PI;
        // player.dx = cos(player.angle)*5;
        // player.dy = sin(player.angle)*5;
    } 
}
// -! Navigation buttons !-
