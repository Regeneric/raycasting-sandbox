#undef _GLIBCXX_DEBUG                			// Disable run-time bound checking, etc
#pragma GCC optimize("Ofast,inline")			// Ofast = O3,fast-math,allow-store-data-races,no-protect-parens


#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#include "pico/stdlib.h"
#include "pico/multicore.h"
#include "pico/binary_info.h"

#include "hardware/flash.h"
#include "hardware/gpio.h"
#include "hardware/irq.h"
#include "hardware/i2c.h"
#include "hardware/timer.h"
#include "hardware/sync.h"

#include "commons.h"
#include "oled.h"


#define BLACK 0x00
#define WHITE 0xFF
#define DR 0.0174533        // One degree in radians


typedef struct Ray {
    float angle;
    float dist;
} Ray; Ray ray;

typedef struct Player {
    float fov;
    float x; float y;
    int size, speed;

    float dx, dy;   // Delta X and Y
    float angle;    // Heading
} Player; Player player;

typedef struct MapConfig {
    int mapX; int mapY;
    int cell;

    int *map;
    int wallpaint;
} MapConfig; MapConfig mapConfig;

int map[] = {
        0,1,1,1,0,0,1,1,1,0,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,1,1,0,0,0,1,
        1,0,0,0,1,1,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,1,1,0,0,0,1,
        1,0,0,0,1,1,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        0,1,1,1,0,0,1,1,1,0,
    };


void sceneInit() {
    player.fov = 30.0f;
    player.size = 2;
    player.x = 15.0f;
    player.y = 15.0f;
    player.speed = 1;   // Speed in pixels

    player.dx = cos(player.angle)*5;
    player.dy = sin(player.angle)*5;
    player.angle = 90.0f;


    int mx = 10;
    int my = 20;
    
    mapConfig.wallpaint = 0xF;
    mapConfig.mapX = mx;
    mapConfig.mapY = my;
    mapConfig.cell = 8;
    // mapConfig.map = (int*)malloc((mx*my) * sizeof(int));
    mapConfig.map = map;
}

void drawMap() {
    int *map = mapConfig.map;
    int mx = mapConfig.mapX;
    int my = mapConfig.mapY;

    for(int y = 0; y < my; y++) {
        for(int x = 0; x < mx; x++) {
            int c = mapConfig.cell;
            int xo = x * c;
            int yo = y * c;

            if(map[y * mx + x] == 1) oled.rect(xo,yo, c,c,  WHITE);
        }
    }
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
    
    for(int r = 0; r < fov; r += rayStep) {
        // Horizontal line
        float distH = INFINITY;
        float horX = playerX, horY = playerY;

        dof = 0;
        float aTan = -1/tan(rayAngle);


        if(rayAngle > M_PI) {
            rayY = (((int)playerY/cell)*cell) - 0.0001;    // ((player.angle/8)*8) - 0.001
            rayX = (playerY-rayY) * aTan+playerX;

            offsetY = -cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle < M_PI) {
            rayY = (((int)playerY/cell)*cell) + cell;    // ((player.angle/8)*8) + 8
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
            mapX = (int)(rayX)/cell;  // rayX/8
            mapY = (int)(rayY)/cell;  // rayY/8
            mapPos = mapY * mapW + mapX;

            // Wall hit
            if(mapPos > 0  &&  mapPos < mapW*mapH  &&  mapGrid[mapPos] > 0) {
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
        float distV = INFINITY;
        float vertX = playerX, vertY = playerY;

        dof = 0;
        float nTan = -tan(rayAngle);


        if(rayAngle > M_PI_2 && rayAngle < 3*M_PI_2) {
            rayX = (((int)playerX/cell)*3) - 0.0001;
            rayY = (playerX-rayX)*nTan+playerY;

            offsetX = -cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle < M_PI_2 || rayAngle > 3*M_PI_2) {
            rayX = (((int)playerX/cell)*cell) + cell;
            rayY = (playerX-rayX)*nTan+playerY;

            offsetX =  cell;
            offsetY = -(offsetX/2)*nTan;
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

        // oled.line(playerX, playerY,  rayX, rayY,  WHITE);

        int wallpaint = 0x0;
        if(distV < distH) {
            ray.dist = distV;
            rayX = vertX;
            rayY = vertY;

            wallpaint = 0xF;
        }
        if(distH < distV) {
            ray.dist = distH;
            rayX = horX;
            rayY = horY;

            wallpaint = 0x10;
        }


        // oled.line(playerX, playerY,  rayX, rayY,  0x10);
        
        rayAngle += DR;
        if(rayAngle < 0)      rayAngle += 2*M_PI;
        if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;

        ray.angle = rayAngle;
        mapConfig.wallpaint = wallpaint;
    }
}


void drawWalls(float fov, int color) {
    for(int r = 0; r < 3; r++) {
        // 3D walls
        // Removes fish eye effect
        float cellAngle = player.angle - ray.angle;
        if(cellAngle < 0)       cellAngle += 2*M_PI;
        if(cellAngle > 2*M_PI)  cellAngle -= 2*M_PI;
        ray.dist *= cos(cellAngle);


        float lineH = (mapConfig.cell*60)/ray.dist;

        float wallWidth = 120/mapConfig.cell;
        float lineOffset = 60 - lineH/2;

        oled.rect(r*wallWidth, lineOffset,  lineH+lineOffset, r*wallWidth,  mapConfig.wallpaint);
        // for(int x = 0; x < lineH+lineOffset; x++) oled.line(r*wallWidth, lineOffset,  r*wallWidth, x,  WHITE);
        // oled.line(r*wallWidth, lineOffset,  r*wallWidth, lineH+lineOffset,  WHITE);
    }
}

void drawPlayer() {
    oled.rect(player.x, player.y,  player.size, player.size,  0xD);
}


void show();    // Core 1


int main() {
    stdio_init_all();

    gpio_init(21);
    gpio_init(22);

    gpio_set_dir(21, GPIO_IN);
    gpio_set_dir(22, GPIO_IN);

    gpio_pull_up(21);
    gpio_pull_up(22);

    sceneInit();

    // OLED init    
    oled.init();     
    oled.clear(BLACK);
    oled.clear(WHITE);
    oled.clear(BLACK);
    // oled.off();

    multicore_launch_core1(show);

    while(1) {       
        if(gpio_get(21) == 0) {
            player.x += player.dx;
            player.y += player.dy;
            // oled.clear(BLACK);
        }

        if(gpio_get(22) == 0) {
            player.angle += 0.05;
            if(player.angle > 2*M_PI) player.angle -= 2*M_PI;
            player.dx = cos(player.angle)*5;
            player.dy = sin(player.angle)*5;

            // oled.clear(BLACK);
        }

        castRays(player.fov);
        tight_loop_contents();
    } return 0;
}


// Core 1
// ------------------------------------------------------------
void show() {
    while(1) {
        // drawMap();
        // drawPlayer();
        drawWalls(player.fov, mapConfig.wallpaint);
        
        oled.display();
    } return;
}
// Core 1
// ------------------------------------------------------------