#include "headers/Player.hpp"

void Player::move(char c, float delta) {
    switch(c) {
            case 'a': rotAngle -= (0.35f) * delta; break;
            case 'd': rotAngle += (0.35f) * delta; break;
            case 'w':
                posX += sinf(rotAngle) * 2.0f * delta;
                posY += cosf(rotAngle) * 2.0f * delta;

                // if(map.playground[(int)posY * 1 + (int)posX] == '#') {
                //     posX -= sinf(rotAngle) * 2.0f * delta;
                //     posY -= cosf(rotAngle) * 2.0f * delta;
                // }
            break;
            case 's':
                posX -= sinf(rotAngle) * 2.0f * delta;
                posY -= cosf(rotAngle) * 2.0f * delta;

                // if(map.playground[(int)posY * 1 + (int)posX] == '#') {
                //     posX += sinf(rotAngle) * 2.0f * delta;
                //     posY += cosf(rotAngle) * 2.0f * delta;
                // }
            break;
    }
}

void Player::raycast(const Screen *screen) {
    for(int x = 0; x < screen->width; x++) {
        float rayAngle = (rotAngle - fov/2.0f) + ((float)x / (float)screen->width) * fov;

        float wallDistance = 0;
        bool wallHit = false;
        bool wallBoundry = false;

        float eyeX = sinf(rayAngle);
        float eyeY = cosf(rayAngle);

        while(!wallHit && wallDistance < drawDistance) {
            wallDistance += 0.1f;

            int testX = (int)(posX + eyeX*wallDistance);
            int testY = (int)(posY + eyeY*wallDistance);

            // Test for rays out of bounds
            if(testX < 0 || testX >= map->width || testY < 0 || testY >= map->height) {
                wallHit = true;
                wallDistance = player.depth;    // Maximum distance
            } else {
                // Test to see if the ray cell is a wall
                if(playground[testY*map.width + testX] == '#') {
                    wallHit = true;

                    // Cell boundries
                    std::vector<std::pair<float, float>> p;
                    for(int tx = 0; tx <= 2; tx++) {
                        for(int ty = 0; ty <= 2; ty++) {
                            float vy = (float)testY + ty - player.posY;
                            float vx = (float)testX + tx - player.posX;

                            float d = sqrt(vx*vx + vy*vy);
                            float dot = (eyeX*vx / d) + (eyeY*vy / d);

                            p.push_back(std::make_pair(d, dot));
                        }

                        // Sort pairs based on distance from player
                        std::sort(p.begin(), p.end(), 
                            [](const std::pair<float, float> &left, const std::pair<float, float> &right) {
                                return left.first < right.first;
                            });


                        // We're checking how small is the angle
                        // If smaller than `bound` we assume that ray hit the wall boundry
                        float bound = 0.005f;
                        if(acos(p.at(0).second) < bound) wallBoundry = true;
                        if(acos(p.at(1).second) < bound) wallBoundry = true;
                        if(acos(p.at(2).second) < bound) wallBoundry = true;
                    }
                }
            }
        }

        // Distance to ceiling and floor
        map.ceiling = (float)(screen.height/2.0f) - screen.height / ((float)wallDistance);
        map.floor = screen.height - map.ceiling;

        for(int y = 0; y < screen.height; y++) {
            if(y <= map.ceiling) mvprintw(y, x, " "); 
            else if(y > map.ceiling && y < map.floor) {
                if(wallDistance <= player.depth/4.0f)     attron(COLOR_PAIR(1));    // Close
                else if(wallDistance < player.depth/3.0f) attron(COLOR_PAIR(2));
                else if(wallDistance < player.depth/2.0f) attron(COLOR_PAIR(3));
                else if(wallDistance < player.depth)      attron(COLOR_PAIR(4));
                else {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}                  // Far

                if(wallBoundry) mvprintw(y, x, "|");
                else if(wallDistance > player.depth) {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}
                else mvprintw(y, x, "#");

            } else {
                float b = 1.0f - (((float)y - screen.height/2.0f) / ((float)screen.height/2.0f));
                
                if(b < 0.25)      {attron(COLOR_PAIR(2)); mvprintw(y, x, "x");}
                else if(b < 0.5)  {attron(COLOR_PAIR(3)); mvprintw(y, x, "*");}
                else if(b < 0.75) {attron(COLOR_PAIR(4)); mvprintw(y, x, "-");}
                else if(b < 0.9)  {attron(COLOR_PAIR(4)); mvprintw(y, x, ".");}
                else {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}
            }

            // refresh();
        }

        // Framerate and player position
        mvprintw(0, 0, "X=%3.2f, Y=%3.2f, A=%3.2f, FPS=%3.2f", player.posX, player.posY, player.posA, 1.0f/et);

        // Minimap
        for(int mx = 0; mx < map.width; mx++) {
            for(int my = 0; my < map.height; my++) {
                char c = playground[my*map.width + mx];
                mvprintw(my+1, mx, &c);
            }
        } mvprintw((int)player.posY+1, (int)player.posX, "P");

        refresh();
    }
}