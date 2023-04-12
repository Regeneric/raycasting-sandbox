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

#include <cmath>


#define BACK_BTN    (1<<PD5)
#define FORWARD_BTN (1<<PD6)
#define RIGHT_BTN   (1<<PD7)
#define LEFT_BTN    (1<<PB0)

#define BAK_INT_BTN (1<<PCINT21)
#define FWD_INT_BTN (1<<PCINT22)
#define RT_INT_BTN  (1<<PCINT23)
#define LT_INT_BTN  (1<<PCINT0)



template<size_t dp>
constexpr int16_t toFixed(float d) {
    return int16_t(d * float(1<<dp) + (d >= 0 ? 0.5 : -0.5));
}

template<size_t dp>
constexpr float toFloat(int16_t d) {
    return float(d) / float(1<<dp);
}



constexpr int16_t DR      = toFixed<8>(0.0174533);
constexpr int16_t PI      = toFixed<8>(M_PI);
constexpr int16_t PI_2    = toFixed<8>(2*M_PI);
constexpr int16_t PI_F2   = toFixed<8>(M_2_PI);
constexpr int16_t PI_3_F2 = toFixed<8>(3*M_2_PI);


struct Player {
    int16_t x = toFixed<8>(13.0f); 
    int16_t y = toFixed<8>(13.0f); 
    
    const uint8_t size  = 2; 
    const uint8_t speed = 1;

    int16_t angle = toFixed<8>(260.0f); 
    int16_t dx = toFixed<8>(cos(angle)*5.0f); 
    int16_t dy = toFixed<8>(sin(angle)*5.0f);
}; Player player;

struct MapConfig {
    const uint8_t mapX = 12; 
    const uint8_t mapY = 8;
    const uint8_t cell = 8;

    uint8_t map[12*8] = {
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
int16_t sdist(int16_t x1, int16_t y1, int16_t x2, int16_t y2) {return sqrt((x2-x1)*(x2-x1)  +  (y2-y1)*(y2-y1));}

void castRays() {
    uint8_t dof = 0; 
    int16_t rayX = 0, rayY = 0, rayAngle = 0;
    constexpr uint8_t fov = 60;

    constexpr uint8_t cell = mapConfig.cell, mapW = mapConfig.mapX, mapH = mapConfig.mapY;
    uint8_t mapX = 0, mapY = 0;
    int16_t mapPos = 0;
    int16_t offsetX = 0, offsetY = 0;

    int16_t playerX = player.x;
    int16_t playerY = player.y;
    int16_t playerAngle = player.angle - toFixed<8>((fov/2));

    constexpr int16_t rounding = 9;     // Magic number  -  comes from int32_t(0.00015f * float(1<<16));


    // constexpr int16_t a = toFixed<8>(5.6);
    // constexpr int16_t b = toFixed<8>(2.7);

    // constexpr int16_t c = a + b;
    // constexpr int16_t d = a - b;

    // constexpr int16_t e = (int32_t(a) * int32_t(b)) >> 8;
    // constexpr int16_t fe = (int32_t(a) << 8) / int32_t(b);


    // Ray angle in radians
    rayAngle = playerAngle;
    if(rayAngle < 0)    rayAngle += PI_2;
    if(rayAngle > PI_2) rayAngle -= PI_2;
    
    cli();
    for(uint8_t r = 0; r < fov; r++) {
        // Horizontal line
        int16_t distH = 1000000;
        int16_t horX = playerX, horY = playerY;

        dof = 0;
        int16_t aTan = toFixed<8>(-1/tan(rayAngle));

        if(rayAngle > PI) {
            int16_t pd = (int32_t(playerY) << 8) / int32_t(cell);  // pd = playerY / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayY = pm - rounding;
            rayX = (playerY-rayY) * aTan + playerX;

            offsetY = -cell;
            offsetX = -offsetY * aTan;
        }
        if(rayAngle < PI) {
            int16_t pd = (int32_t(playerY) << 8) / int32_t(cell);  // pd = playerY / cell
            int16_t pm = (int32_t(pm) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayY = pm + cell;
            rayX = (playerY-rayY) * aTan + playerX;

            offsetY = cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle == 0 || rayAngle == PI) {
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
        int16_t nTan = toFixed<8>(-tan(rayAngle));


        if(rayAngle > PI_F2 && rayAngle < PI_3_F2) {
            int16_t pd = (int32_t(playerX) << 8) / int32_t(cell);  // pd = playerX / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayX = pm - rounding;
            rayY = (playerX-rayX) * nTan + playerY;

            offsetX = -cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle < PI_2 || rayAngle > PI_3_F2) {
            int16_t pd = (int32_t(playerX) << 8) / int32_t(cell);  // pd = playerX / cell
            int16_t pm = (int32_t(pd) * int32_t(cell)) >> 8;       // pm = pd * cell

            rayX = pm + cell;
            rayY = (playerX-rayX) * nTan + playerY;

            offsetX =  cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle == 0 || rayAngle == PI) {
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
        if(rayAngle < 0)    rayAngle += PI_2;
        if(rayAngle > PI_2) rayAngle -= PI_2;
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

        player.angle -= toFixed<8>(0.05);
        if(player.angle < 0) player.angle += PI_2;
        player.dx = toFixed<8>(cos(player.angle)*5);
        player.dy = toFixed<8>(sin(player.angle)*5);

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

        player.angle += toFixed<8>(0.05);
        if(player.angle > PI_2) player.angle -= PI_2;
        player.dx = toFixed<8>(cos(player.angle)*5);
        player.dy = toFixed<8>(sin(player.angle)*5);

        // player.angle += 0.05;
        // if(player.angle > 2*M_PI) player.angle -= 2*M_PI;
        // player.dx = cos(player.angle)*5;
        // player.dy = sin(player.angle)*5;
    } 
}
// -! Navigation buttons !-
