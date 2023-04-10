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


#define BACK_BTN    (1<<PD5)
#define FORWARD_BTN (1<<PD6)
#define RIGHT_BTN   (1<<PD7)
#define LEFT_BTN    (1<<PB0)

#define BAK_INT_BTN (1<<PCINT21)
#define FWD_INT_BTN (1<<PCINT22)
#define RT_INT_BTN  (1<<PCINT23)
#define LT_INT_BTN  (1<<PCINT0)


typedef struct Player {
    float x; float y;
    int size, speed;

    float dx, dy;   // Delta X and Y
    float angle;    // Heading
} Player; Player player;

typedef struct MapConfig {
    int mapX; int mapY;
    int cell;

    int *map;
} MapConfig; MapConfig mapConfig;

static const int map[] = {
    1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,0,1,0,0,0,0,1,1,
    1,0,0,0,0,0,0,0,0,1,1,1,
    1,0,0,0,1,0,0,1,0,0,1,1,
    1,0,0,0,0,0,0,0,0,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,
};



void sceneInit() {
    player.size = 2;
    player.x = 12.0f;
    player.y = 10.0f;
    player.speed = 1;   // Speed in pixels

    player.dx = cos(player.angle)*5;
    player.dy = sin(player.angle)*5;
    player.angle = 90.0f;


    int mx = 12;
    int my = 8;

    mapConfig.mapX = mx;
    mapConfig.mapY = my;
    mapConfig.cell = 8;
    // mapConfig.map = (int*)malloc((mx*my) * sizeof(int));
    mapConfig.map = map;
}

void drawMap() {
    cli();

    int *map = mapConfig.map;
    int mx = mapConfig.mapX;
    int my = mapConfig.mapY;

    for(int y = 0; y < my; y++) {
        for(int x = 0; x < mx; x++) {
            int c = mapConfig.cell;
            int xo = x * c;
            int yo = y * c;

            if(map[y * mx + x] == 1) {LCD.rect(xo,yo, c,c);}
        }
    }

    sei();
}


float sdist(float x1, float y1, float x2, float y2) {return sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1));}

void castRays(float f) {
    float dist = 0.0f;

    int dof = 0, fov = f;
    float rayX = 0.0f, rayY = 0.0f, rayAngle = 0.0f;
    int rayStep = 1;

    int cell = mapConfig.cell, mapW = mapConfig.mapX, mapH = mapConfig.mapY, mapTV = 0, mapTH = 0;
    int mapX = 0.0f, mapY = 0.0f, mapPos = 0.0f;
    float offsetX = 0.0f, offsetY = 0.0f;
    int *mapGrid = mapConfig.map;

    float playerX = player.x;
    float playerY = player.y;
    float playerAngle = player.angle - (fov/2);

    // Ray angle in radians
    rayAngle = playerAngle;
    if(rayAngle < 0)      rayAngle += 2*M_PI;
    if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;
    
    cli();
    for(int r = 0; r < fov; r += rayStep) {
        // Horizontal line
        float distH = 1000000;
        float horX = playerX, horY = playerY;

        dof = 0;
        float aTan = -1/tan(rayAngle);


        if(rayAngle > M_PI) {
            rayY = (((int)playerY>>3)<<3) - 0.0001;    // ((player.angle/8)*8) - 0.001
            rayX = (playerY-rayY) * aTan+playerX;

            offsetY = -cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle < M_PI) {
            rayY = (((int)playerY>>3)<<3) + cell;    // ((player.angle/8)*8) + 8
            rayX = (playerY-rayY) * aTan+playerX;

            offsetY = cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle == 0 || rayAngle == M_PI) {
            rayX = playerX;
            rayY = playerY;
            dof = 8;
        }


        while(dof < 8) {
            mapX = (int)(rayX)>>3;  // rayX/8
            mapY = (int)(rayY)>>3;  // rayY/8
            mapPos = mapY * mapW + mapX;

            // Wall hit
            if(mapPos < mapW*mapH  &&  mapGrid[mapPos] > 0) {
                horX = rayX;
                horY = rayY;
                distH = sdist(playerX, playerY, horY, horX);

                dof = 8;
            } else {
                rayX += offsetX;
                rayY += offsetY;
                dof += 1;
            }
        }


        // Vertical line
        float distV = 1000000;
        float vertX = playerX, vertY = playerY;

        dof = 0;
        float nTan = -tan(rayAngle);


        if(rayAngle > M_PI_2 && rayAngle < 3*M_PI_2) {
            rayX = (((int)playerX>>3)<<3) - 0.0001;
            rayY = (playerX-rayX)*nTan+playerY;

            offsetX = -cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle < M_PI_2 || rayAngle > 3*M_PI_2) {
            rayX = (((int)playerX>>3)<<3) + cell;
            rayY = (playerX-rayX)*nTan+playerY;

            offsetX =  cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle == 0 || rayAngle == M_PI) {
            rayX = playerX;
            rayY = playerY;
            dof = 8;
        }


        while(dof < 8) {
            mapX = (int)(rayX)/cell;
            mapY = (int)(rayY)/cell;
            mapPos = mapY * mapW + mapX;

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapGrid[mapPos] > 0)) {
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

        LCD.line(playerX, playerY, rayX, rayY);

        if(distV < distH) {
            rayX = vertX;
            rayY = vertY;
        }
        if(distH < distV) {
            rayX = horX;
            rayY = horY;
        }

        LCD.line(playerX, playerY, rayX, rayY);
        
        rayAngle += DR*3;
        if(rayAngle < 0)      rayAngle += 2*M_PI;
        if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;

        // 3D walls
        // Removes fish eye effect
        // float cellAngle = playerAngle - rayAngle;
        // if(cellAngle < 0)       cellAngle += 2*M_PI;
        // if(cellAngle > 2*M_PI)  cellAngle -= 2*M_PI;
        // dist *= cos(cellAngle);


        // float lineH = (cell*70)/dist;

        // float wallWidth = 36/cell;
        // float lineOffset = 25 - lineH/2;

        // LCD.rect(r*wallWidth, lineOffset,  r*wallWidth, lineH+lineOffset);
    }

    sei();
}

void drawPlayer() {
    cli();
    LCD.rect(player.x, player.y,  player.size, player.size);
    castRays(10.0f);
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
    sceneInit();

    sei();
    while(1) {
        drawMap();
        drawPlayer();
        // LCD.line(player.x, player.y,  player.x+player.dx, player.y+player.dy);

        LCD.render();
        LCD.clear();
    } 
    
    // free(mapConfig.map);
    return 0;
}


// -- Navigation buttons --
ISR(PCINT0_vect) {
    if(!(PINB & LEFT_BTN)) {
        // Low state on LEFT_BTN
        // player.x -= player.speed;

        player.angle -= 0.05;
        if(player.angle < 0) player.angle += 2*M_PI;
        player.dx = cos(player.angle)*5;
        player.dy = sin(player.angle)*5;
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

        player.angle += 0.05;
        if(player.angle > 2*M_PI) player.angle -= 2*M_PI;
        player.dx = cos(player.angle)*5;
        player.dy = sin(player.angle)*5;
    } 
}
// -! Navigation buttons !-
