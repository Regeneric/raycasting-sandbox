#include "commons.hpp"
#include "Ray.hpp"
#include "Player.hpp"

#include "SelbaWard/Line.hpp"

#include <iostream>
#include <cmath>
#include <vector>


void Ray::cast(float f, Player &player, std::shared_ptr<Wall> map, std::shared_ptr<sf::RenderWindow> window) {
    static constexpr int texWidth = 64;
    static constexpr int texHeight = 64;
   
    int buffer[WIDTH][HEIGHT]; // y-coordinate first because it works per scanline
    std::vector<int> texture[8];
    for(int i = 0; i < 8; i++) texture[i].resize(texWidth * texHeight);

    for(int x = 0; x < texWidth; x++)
    for(int y = 0; y < texHeight; y++) {
        int xorcolor = (x * 256 / texWidth) ^ (y * 256 / texHeight);
        //int xcolor = x * 256 / texWidth;
        int ycolor = y * 256 / texHeight;
        int xycolor = y * 128 / texHeight + x * 128 / texWidth;
        texture[0][texWidth * y + x] = 65536 * 254 * (x != y && x != texWidth - y); //flat red texture with black cross
        texture[1][texWidth * y + x] = xycolor + 256 * xycolor + 65536 * xycolor; //sloped greyscale
        texture[2][texWidth * y + x] = 256 * xycolor + 65536 * xycolor; //sloped yellow gradient
        texture[3][texWidth * y + x] = xorcolor + 256 * xorcolor + 65536 * xorcolor; //xor greyscale
        texture[4][texWidth * y + x] = 256 * xorcolor; //xor green
        texture[5][texWidth * y + x] = 65536 * 192 * (x % 16 && y % 16); //red bricks
        texture[6][texWidth * y + x] = 65536 * ycolor; //red gradient
        texture[7][texWidth * y + x] = 128 + 256 * 128 + 65536 * 128; //flat grey texture
    }







    float dist = 0.0f;

    int dof = 0, fov = f;
    float rayX = 0.0f, rayY = 0.0f, rayAngle = 0.0f;
    float rayStep = 1.0f;

    int cell = map->cell(), mapW = map->width(), mapH = map->height(), mapTV = 0, mapTH = 0;
    int mapX = 0.0f, mapY = 0.0f, mapPos = 0.0f;
    float offsetX = 0.0f, offsetY = 0.0f;
    std::vector<int> mapGrid = map->grid();

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

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapGrid[mapPos] > 0)) {
                mapTH = mapGrid[mapPos];
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

            if((mapPos > 0) && (mapPos < mapW * mapH) && (mapGrid[mapPos] > 0)) {
                mapTV = mapGrid[mapPos];
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


            distSqr = (dist*dist);
            brightness = hkk::map(distSqr, 0.0f, widthSqr, 255.0f, 0.0f);     // Brightness based wall shading
        
            switch(mapTV) {
                case 1: wallpaint.r = brightness; break;
                case 2: wallpaint.g = brightness; break;
                case 3: wallpaint.b = brightness; break;
                case 4: {wallpaint.r = brightness; wallpaint.g = brightness;} break;
                default: break;
            }
        }
        if(distH < distV) {
            rayX = horX;
            rayY = horY;
            dist = distH;


            distSqr = (dist*dist);
            brightness = hkk::map(distSqr, 0.0f, widthSqr, 230.0f, 0.0f);     // Brightness based wall shading
        
            switch(mapTH) {
                case 1: wallpaint.r = brightness; break;
                case 2: wallpaint.g = brightness; break;
                case 3: wallpaint.b = brightness; break;
                case 4: {wallpaint.r = brightness; wallpaint.g = brightness;} break;
                default: break;
            }
        }

        window->draw(sw::Line(sf::Vector2f(playerX, playerY), sf::Vector2f(rayX, rayY), 0.0f, wallpaint));

        rayAngle += hkk::radians(rayStep);  // Move one ray radian(rayStep) from another

        if(rayAngle < 0)      rayAngle += 2*M_PI;
        if(rayAngle > 2*M_PI) rayAngle -= 2*M_PI;


        // 3D walls
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
        sf::Texture wallTex;
            wallTex.loadFromFile("test_pattern1.bmp");

        sw::Line wall;
            wall.setThickness(wallWidth);
            wall.setPoint(wall.getStartIndex(), {r*wallWidth+530, lineOffset});
            wall.setPoint(wall.getEndIndex()  , {r*wallWidth+530, lineH+lineOffset});
            // wall.setColor(sf::Color(brightness, brightness, brightness, 255));
            wall.setColor(wallpaint);
            wall.setTexture(wallTex);
        window->draw(wall);
    }
}









    // const int mapWidth = 24;
    // const int mapHeight = 24;

    // int worldMap[mapWidth][mapHeight]= {
    //     {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1},
    //     {1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1},
    //     {1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1},
    //     {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1},
    // };

    // sf::Vector2f playerPos = player._player.getPosition();
    // float posX = playerPos.x;
    // float posY = playerPos.y;

    // float dirX = hkk::fromAngle(hkk::radians(player.rotation())).x;
    // float dirY = hkk::fromAngle(hkk::radians(player.rotation())).y;

    // float planeX = hkk::perpendicular(playerPos).x;
    // float planeY = hkk::perpendicular(playerPos).y - 0.34;

    // for(int x = 0; x < WIDTH; x++) {
    //     float cameraX = 2*x / float(WIDTH) - 1;
    //     float rayDirX = dirX + planeX * cameraX;
    //     float rayDirY = dirY + planeY * cameraX;

    //     // int mapX = int(posX)/30;
    //     // int mapY = int(posY)/30;
    //     // int mapPos = mapY * mapWidth + mapX;

    //     int mapX = int(posX/30);
    //     int mapY = int(posY/30);

    //     float sideDistX;
    //     float sideDistY;

    //     float deltaDistX = (rayDirX == 0) ? INFINITY : std::abs(1/rayDirX);
    //     float deltaDistY = (rayDirY == 0) ? INFINITY : std::abs(1/rayDirY);
    //     float perpWallDist;

    //     int stepX;
    //     int stepY;

    //     int hit = 0;
    //     int side;

    //     if(rayDirX < 0) {stepX = -1; sideDistX = (posX - mapX) * deltaDistX;}
    //     else {stepX = 1; sideDistX = (mapX +1.0 - posX) * deltaDistX;}

    //     if(rayDirY < 0) {stepY = -1; sideDistY = (posY - mapY) * deltaDistY;}
    //     else {stepY = 1; sideDistY = (mapY + 1.0 - posY) * deltaDistY;}
        
    //     while(hit == 0) {
    //         if(sideDistX < sideDistY) {
    //             sideDistX += deltaDistX;
    //             mapX += stepX;
    //             side = 0;
    //         } else {
    //             sideDistY += deltaDistY;
    //             mapY += stepY;
    //             side = 1;
    //         }

    //         if(worldMap[mapX][mapY] > 0) hit = 1;
    //     }

    //     // Calculating distance from camera plane - no need to "fisheye" correction
    //     if(side == 0) perpWallDist = (sideDistX - deltaDistX);
    //     else          perpWallDist = (sideDistY - deltaDistY);

    //     int lineHeight = (int)(HEIGHT / perpWallDist);
        
    //     int drawStart = -lineHeight / 2+HEIGHT / 2;
    //     if(drawStart < 0) drawStart = 0;

    //     int drawEnd = lineHeight / 2+HEIGHT / 2;
    //     if(drawEnd >= HEIGHT) drawEnd = HEIGHT-1;

    //     sf::Color wallpaint;
    //     switch(worldMap[mapX][mapY]) {
    //         case 1:  wallpaint = sf::Color::Red;    break;
    //         case 2:  wallpaint = sf::Color::Green;  break;
    //         case 3:  wallpaint = sf::Color::Blue;   break;
    //         case 4:  wallpaint = sf::Color::White;  break;
    //         default: wallpaint = sf::Color::Yellow; break;
    //     }

    //     if(side == 1) {
    //         wallpaint.r /= 2;
    //         wallpaint.g /= 2;
    //         wallpaint.b /= 2;
    //     }

    //     sw::Line wall;
    //         wall.setThickness(0.0f);
    //         wall.setPoint(wall.getStartIndex(), {x, drawStart});
    //         wall.setPoint(wall.getEndIndex()  , {x, drawEnd});
    //         wall.setColor(wallpaint);
    //     window->draw(wall);
    // }


    // std::cout << "PLX: " << playerX << " ; PLY: " << planeY << std::endl;
    // std::cout << "DRX: " << dirX << " ; DRY: " << dirY << std::endl;
    // std::cout << "PPX: " << planeX << " ; PPY: " << planeY << std::endl;
    // std::cout << "-------------------------------------------" << std::endl;

