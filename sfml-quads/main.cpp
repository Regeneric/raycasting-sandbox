#include <SFML/Graphics.hpp>

#include "commons.hpp"

#include <iostream>
#include <optional>

#define DR hkk::radians(1)  // One degree in radians

void cast(float pa, sf::Vector2f pos, hkk::MapConfig config, std::vector<int> map, sf::RenderWindow *window) {
    float dist = 0.0;

    int r = 0, dof = 0, fov = 60.0;
    float rx = 0.0f, ry = 0.0f, ra = 0.0f;

    int mx = 0, my = 0, mp = 0;
    float xo = 0.0f, yo = 0.0f;

    float px = pos.x;
    float py = pos.y;

    ra = pa - hkk::radians(fov/2); 
    if(ra < 0)      {ra += 2*M_PI;}
    if(ra > 2*M_PI) {ra -= 2*M_PI;}

    for(int r = 0; r < fov; r++) {
        // Horizontal line
        float distH = INFINITY;
        float hx = px, hy = py;

        dof = 0.0f;
        float aTan = -1/tan(ra);

        if(ra > M_PI) {ry = (((int)py/config.cell)*config.cell)-0.0001;      rx = (py-ry)*aTan+px; yo = -config.cell; xo = -yo*aTan;}
        if(ra < M_PI) {ry = (((int)py/config.cell)*config.cell)+config.cell; rx = (py-ry)*aTan+px; yo =  config.cell; xo = -yo*aTan;}
        if(ra == 0 || ra == M_PI) {rx = px; ry = py; dof = 8;}

        while(dof < 8) {
            mx = (int)(rx)/config.cell;
            my = (int)(ry)/config.cell;
            mp = my*config.width+mx;

            if(mp > 0  &&  mp < config.width*config.height  &&  map[mp] == 1) {
                hx = rx;
                hy = ry;
                distH = hkk::dist(px, py, hx, hy);

                dof = 8;
            } else {rx += xo; ry += yo; dof += 1;}
        }


        // Vertical line
        float distV = INFINITY;
        float vx = px, vy = py;

        dof = 0.0f;
        float nTan = -tan(ra);

        if(ra > M_PI_2 && ra < 3*M_PI_2) {rx = (((int)px/config.cell)*config.cell)-0.0001;      ry = (px-rx)*nTan+py; xo = -config.cell; yo = -xo*nTan;}
        if(ra < M_PI_2 || ra > 3*M_PI_2) {rx = (((int)px/config.cell)*config.cell)+config.cell; ry = (px-rx)*nTan+py; xo =  config.cell; yo = -xo*nTan;}
        if(ra == 0 || ra == M_PI) {rx = px; ry = py; dof = 8;}

        while(dof < 8) {
            mx = (int)(rx)/config.cell;
            my = (int)(ry)/config.cell;
            mp = my*config.width+mx;

            if(mp > 0  &&  mp < config.width*config.height  &&  map[mp] == 1) {
                vx = rx;
                vy = ry;
                distV = hkk::dist(px, py, vx, vy);
            
                dof = 8;
            } else {rx += xo; ry += yo; dof += 1;}
        }


        if(distV < distH) {rx = vx; ry = vy; dist = distV;}
        if(distH < distV) {rx = hx; ry = hy; dist = distH;}

        hkk::Line l(px, py, rx, ry);
        window->draw(l.line);

        // 3D walls
        // ra += hkk::radians(0.15);
        ra += DR;

        if(ra < 0)      {ra += 2*M_PI;}
        if(ra > 2*M_PI) {ra -= 2*M_PI;}
    
        // float lineH = (config.cell * WIDTH-50)/dist; if(lineH > WIDTH-50) lineH = WIDTH-50;
        float ca = pa - ra; 
        if(ca < 0)      {ca += 2*M_PI;}
        if(ca > 2*M_PI) {ca -= 2*M_PI;}
        dist *= cos(ca);

        
        float lineH = hkk::map((double)dist, 0.0, (double)WIDTH, (double)HEIGHT, 0.0);
        float lineO = lineH/2;

        double sq = (double)(dist*dist);
        double wq = (double)(WIDTH*WIDTH);
        float brightness = hkk::map(sq, 0.0, wq, 255.0, 0.0);

        sf::Transform t;
        t.translate(WIDTH-45, 0);

        sf::RectangleShape wall;
            wall.setFillColor(sf::Color(brightness, brightness, brightness, 255));
            wall.setPosition(sf::Vector2f(r*8 + map.size()/2, HEIGHT/2));
            wall.setSize(sf::Vector2f(8, lineH));
            wall.setOrigin(wall.getSize().x/2, wall.getSize().y/2);
    
        window->draw(wall, t);
    }
}

int main() {
    sf::RenderWindow window(sf::VideoMode(800, HEIGHT), "Ray Marching", sf::Style::Close);
    window.setView(sf::View(sf::FloatRect(0, 0, 800, HEIGHT)));


    std::vector<int> map {
        1,1,1,1,1,1,1,1,1,1,
        1,0,0,0,0,0,0,0,0,1,
        1,1,1,1,0,0,1,1,0,1,
        1,0,1,1,0,0,0,1,1,1,
        1,0,0,1,0,0,0,0,0,1,
        1,0,0,0,0,0,0,0,0,1,
        1,0,0,0,0,0,1,0,0,1,
        1,0,1,1,0,0,0,0,0,1,
        1,0,1,1,0,0,1,0,0,1,
        1,1,1,1,1,1,1,1,1,1,
    }; hkk::MapConfig mapConfig;


    sf::RectangleShape player;
        player.setFillColor(sf::Color::Red);
        player.setSize(sf::Vector2f(10.0f, 10.0f));
        player.setPosition(sf::Vector2f(WIDTH/2, HEIGHT/2));
        player.setOrigin(player.getSize().x/2, player.getSize().y/2);
    float pa = player.getRotation();
    sf::Vector2f pd(0.0f, -1.0f);

    while(window.isOpen()) {
        sf::Event e;
        while(window.pollEvent(e)) {
            switch(e.type) {case sf::Event::Closed: window.close(); break;}
        } window.clear(sf::Color(80, 80, 80));
        
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {player.rotate( 0.15f);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {player.rotate(-0.15f);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {player.move( hkk::fromAngle(hkk::radians(player.getRotation())));}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {player.move(-hkk::fromAngle(hkk::radians(player.getRotation())));}


        for(int y = 0; y < mapConfig.height; y++) {
            for(int x = 0; x < mapConfig.width; x++) {
                sf::RectangleShape wall;
                if(map[y*mapConfig.width+x] == 1) wall.setFillColor(sf::Color::Black);
                else wall.setFillColor(sf::Color::White);

                int xs = x*mapConfig.cell;
                int ys = y*mapConfig.cell;

                wall.setSize(sf::Vector2f(mapConfig.cell-1, mapConfig.cell-1));
                wall.setPosition(sf::Vector2f(xs, ys));
                window.draw(wall);   
            }
        }
        
        window.draw(player);
        cast(hkk::radians(player.getRotation()), player.getPosition(), mapConfig, map, &window);

        window.display();
    } return 0;
}