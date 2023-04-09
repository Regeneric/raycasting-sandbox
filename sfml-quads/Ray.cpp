#include "commons.hpp"
#include "Ray.hpp"
#include "Player.hpp"

#include "SelbaWard/Line.hpp"

#include <iostream>

void Ray::cast(float f, Player &player, Wall map, sf::RenderWindow *window) {
    float dist = 0.0f;

    int dof = 0, fov = f;
    float rayX = 0.0f, rayY = 0.0f, rayAngle = 0.0f;
    float rayStep = 0.75f;

    int cell = map.cell(), mapW = map.width(), mapH = map.height();
    int mapX = 0.0f, mapY = 0.0f, mapPos = 0.0f;
    float offsetX = 0.0f, offsetY = 0.0f;
    std::vector<int> mapGrid = map.grid();

    float playerX = player.position().x;
    float playerY = player.position().y;
    float playerAngle = hkk::radians(player.rotation());

    // Ray angle in radians
    rayAngle = playerAngle - hkk::radians(fov/2);
    if(rayAngle < 0)      rayAngle += 2*M_PI;
    if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;

    for(float r = 0; r < fov; r += rayStep) {
        // Horizontal line
        float distH = INFINITY;
        float horX = playerX, horY = playerY;

        dof = 0;
        float aTan = -1/tan(rayAngle);


        if(rayAngle > M_PI) {
            rayY = (((int)playerY/cell)*cell) - 0.0001;     // Rounding to closest cell
            rayX = (playerY-rayY)*aTan+playerX;
        
            offsetY = -cell;
            offsetX = -offsetY*aTan;
        }
        if(rayAngle < M_PI) {
            rayY = (((int)playerY/cell)*cell) + cell;       // Rounding to closest cell
            rayX = (playerY - rayY)*aTan+playerX;

            offsetY =  cell;
            offsetX = -offsetY*aTan;
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

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapGrid[mapPos] == 1)) {
                horX = rayX;
                horY = rayY;
                distH = hkk::dist(playerX, playerY, horX, horY);

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
            rayX = (((int)playerX/cell)*cell) - 0.0001;
            rayY = (playerX-rayX)*nTan+playerY;

            offsetX = -cell;
            offsetY = -offsetX*nTan;
        }
        if(rayAngle < M_PI_2 || rayAngle > 3*M_PI_2) {
            rayX = (((int)playerX/cell)*cell) + cell;
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

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapGrid[mapPos] == 1)) {
                vertX = rayX;
                vertY = rayY;
                distV = hkk::dist(playerX, playerY, vertX, vertY);

                dof = 8;
            } else {
                rayX += offsetX;
                rayY += offsetY;
                dof += 1;
            }
        }

        int shade = 0;
        sf::Color wallpaint;
        
        float distSqr = 0.0f;
        float brightness = 0.0f;
        float widthSqr = (WIDTH*WIDTH);

        if(distV < distH) {
            rayX = vertX;
            rayY = vertY;
            dist = distV;
            
            wallpaint.r = 230;
            distSqr = (dist*dist);
            brightness = hkk::map(distSqr, 0.0f, widthSqr, 255.0f, 0.0f);     // Brightness based wall shading
        }
        if(distH < distV) {
            rayX = horX;
            rayY = horY;
            dist = distH;

            wallpaint.r = 179;
            distSqr = (dist*dist);
            brightness = hkk::map(distSqr, 0.0f, widthSqr, 230.0f, 0.0f);     // Brightness based wall shading
        }

        if(window != nullptr) window->draw(hkk::Line(playerX, playerY, rayX, rayY).line);


        // 3D walls
        rayAngle += hkk::radians(rayStep);  // Move one ray radian(rayStep) from another

        if(rayAngle < 0)      rayAngle += 2*M_PI;
        if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;

        // Removes fish eye effect
        float cellAngle = playerAngle - rayAngle;
        if(cellAngle < 0)       cellAngle += 2*M_PI;
        if(cellAngle > 2*M_PI)  cellAngle -= 2*M_PI;
        dist *= cos(cellAngle);


        float lineH = (cell*400)/dist;
        // if(lineH > 320) lineH = 320;

        // Set brightness between bounds
        if(brightness < 0)   brightness = 0;
        if(brightness > 255) brightness = 255;

        float wallWidth = WIDTH/cell;
        float lineOffset = 240 - lineH/2;

        // Draw walls with thick lines
        sw::Line wall;
            wall.setThickness(wallWidth);
            wall.setPoint(wall.getStartIndex(), {r*wallWidth+530, lineOffset});
            wall.setPoint(wall.getEndIndex()  , {r*wallWidth+530, lineH+lineOffset});
            wall.setColor(sf::Color(brightness, brightness, brightness, 255));
            // wall.setColor(wallpaint);
        window->draw(wall);
    }
}