#include <ncurses.h>

#include <iostream>
#include <string>
#include <chrono>
#include <vector>
#include <algorithm>

#include <cmath>
#include <clocale>

struct {
    int width;
    int height;
} screen{.width = 120, .height = 40};

struct {
    float posX;
    float posY;
    float posA;

    float fov;
    float depth;
} player{.posX = 8.0f, .posY = 8.0f, .posA = 0.0f, .fov = M_PI/4.0f, .depth = 16.0f};

struct {
    int width;
    int height;

    int ceiling;
    int floor;
} map{.width = 32, .height = 16};


int main() {
    setlocale(LC_ALL, "");

    initscr();
    resize_term(screen.height, screen.width);
    refresh();
    noecho();
    curs_set(0);
    start_color();
    

    init_color(COLOR_MAGENTA, 10, 10, 10);
    init_color(COLOR_CYAN, 200, 200, 200);
    init_color(COLOR_YELLOW, 400, 400, 400);
    init_color(COLOR_BLUE, 600, 600, 600);
    init_color(COLOR_GREEN, 800, 800, 800);


    init_pair(6, COLOR_RED    , COLOR_BLACK);
    init_pair(5, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(4, COLOR_CYAN   , COLOR_BLACK);
    init_pair(3, COLOR_YELLOW , COLOR_BLACK);
    init_pair(2, COLOR_BLUE   , COLOR_BLACK);
    init_pair(1, COLOR_GREEN  , COLOR_BLACK);


    std::string playground;
        playground += "############    ###########     ";
        playground += "#...............#..............#";
        playground += "#...#....########......#########";
        playground += "#..............##..#.......#...#";
        playground += "#..#....##.....##.....##.......#";
        playground += "#.......##............##...#...#";
        playground += "#....#.........##..............#";
        playground += "###........#...####....#.......#";
        playground += "##...#.........##..............#";
        playground += "#...##.......####...#...#...####";
        playground += "#..............................#";
        playground += "###..####....#######....########";
        playground += "####.####.......######.........#";
        playground += "#.......#.......#..............#";
        playground += "#..........##.........###......#";
        playground += "#    ######################  ###";


    auto clk1 = std::chrono::system_clock::now();
    auto clk2 = std::chrono::system_clock::now();

    while(1) {
        // Frame independent movement
        clk2 = std::chrono::system_clock::now();
        std::chrono::duration<float> cet = clk2 - clk1;
        clk1 = clk2;
        float et = cet.count();

        switch(getch()) {
            case 'a': player.posA -= (0.35f) * et; break;
            case 'd': player.posA += (0.35f) * et; break;
            case 'w':
                player.posX += sinf(player.posA) * 2.0f * et;
                player.posY += cosf(player.posA) * 2.0f * et;

                if(playground[(int)player.posY * map.width + (int)player.posX] == '#') {
                    player.posX -= sinf(player.posA) * 2.0f * et;
                    player.posY -= cosf(player.posA) * 2.0f * et;
                }
            break;
            case 's':
                player.posX -= sinf(player.posA) * 2.0f * et;
                player.posY -= cosf(player.posA) * 2.0f * et;

                if(playground[(int)player.posY * map.width + (int)player.posX] == '#') {
                    player.posX += sinf(player.posA) * 2.0f * et;
                    player.posY += cosf(player.posA) * 2.0f * et;
                }
            break;
            default: continue; break;
        }


        for(int x = 0; x < screen.width; x++) {
            float rayAngle = (player.posA - player.fov/2.0f) + ((float)x / (float)screen.width) * player.fov;

            float wallDistance = 0;
            bool wallHit = false;
            bool wallBoundry = false;

            float eyeX = sinf(rayAngle);
            float eyeY = cosf(rayAngle);

            while(!wallHit && wallDistance < player.depth) {
                wallDistance += 0.1f;

                int testX = (int)(player.posX + eyeX*wallDistance);
                int testY = (int)(player.posY + eyeY*wallDistance);

                // Test for rays out of bounds
                if(testX < 0 || testX >= map.width || testY < 0 || testY >= map.height) {
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
                    if(wallDistance <= player.depth/4.0f)     attron(COLOR_PAIR(1));  // Close
                    else if(wallDistance < player.depth/3.0f) attron(COLOR_PAIR(2));
                    else if(wallDistance < player.depth/2.0f) attron(COLOR_PAIR(3));
                    else if(wallDistance < player.depth)      attron(COLOR_PAIR(4));
                    else {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}  // Far

                    // if(wallDistance <= player.depth/4.0f)     {attron(COLOR_PAIR(1)); mvaddstr(y, x, "\xe2\x96\x93");}    // Close
                    // else if(wallDistance < player.depth/3.0f) {attron(COLOR_PAIR(2)); mvaddstr(y, x, "\xe2\x96\x93");}
                    // else if(wallDistance < player.depth/2.0f) {attron(COLOR_PAIR(3)); mvaddstr(y, x, "\xe2\x96\x92");}
                    // else if(wallDistance < player.depth)      {attron(COLOR_PAIR(4)); mvaddstr(y, x, "\xe2\x96\x91");}
                    // else {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}  // Far

                    if(wallBoundry) mvaddstr(y, x, "┃"); //mvprintw(y, x, "|");
                    else if(wallDistance > player.depth) {attron(COLOR_PAIR(5)); mvprintw(y, x, " ");}
                    else mvprintw(y, x, "#");
                    // else mvaddstr(y, x, "\xe2\x96\x88");

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
                    char c = playground[my*map.width + mx+2];
                    mvprintw(my+1, mx, &c);
                }
            } mvprintw((int)player.posY+1, (int)player.posX, "P");

            refresh();
        }
    } 
    
    endwin();
    return 0;
}