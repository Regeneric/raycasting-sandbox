#include <SFML/Graphics.hpp>

#include <iostream>
#include <vector>
#include <string>
#include <optional>
#include <random>

#include "Boundry.hpp"
#include "Ray.hpp"
#include "Particle.hpp"
#include "SelbaWard/Line.hpp"


static constexpr int WIDTH = 400;
static constexpr int HEIGHT = 400;


// THIS EXAMPLE DOES NOT UTILIZE RAY MARCHING
// IT'S BASED ONLY ON LINE-LINE INTERSECTION


int main() {
    sf::RenderWindow window(sf::VideoMode(800, HEIGHT), "SFML Raycasting", sf::Style::Close);
        window.setView(sf::View(sf::FloatRect(0, 0, 800, HEIGHT)));   // Viewport


    std::random_device rd;                            // Obtain a random number from hardware
    std::mt19937 gen(rd());                           // Seed the generator
    std::uniform_int_distribution<> rngW(0, WIDTH);   // Define the range
    std::uniform_int_distribution<> rngH(0, HEIGHT);  // Define the range


    std::vector<Boundry> walls;
    for(int i = 0; i != 5; ++i) {
        float x1 = rngW(gen);  float x2 = rngW(gen);
        float y1 = rngH(gen);  float y2 = rngW(gen);

        Boundry wall(x1, y1, x2, y2);
        walls.push_back(wall);
    }

    // Window border
    walls.push_back(Boundry(1, 1, 1, HEIGHT));
    walls.push_back(Boundry(1, 1, WIDTH, 1));
    walls.push_back(Boundry(WIDTH, 0, WIDTH, HEIGHT));
    walls.push_back(Boundry(WIDTH, HEIGHT, 0, HEIGHT));

    Particle dot(WIDTH/2, HEIGHT/2, 0.20, 40);

    sf::Vector2i pixelPos;
    sf::Vector2f worldPos;


    while(window.isOpen()) {
        sf::Event e;
        while(window.pollEvent(e))
            switch(e.type) {case sf::Event::Closed: window.close(); break;}
        // window.clear(sf::Color(80, 80, 80));
        window.clear(sf::Color::Black);

        if(sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {dot.move( 0.25);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {dot.move(-0.25);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {dot.rotate( 0.005);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {dot.rotate(-0.005);}

        pixelPos = sf::Mouse::getPosition(window);
        worldPos = window.mapPixelToCoords(pixelPos);

        dot.draw(&window);
        for(auto wall : walls) {
            wall.draw(&window, hkk::LineShape);
            // rectWall.draw(&window, hkk::RectShape);
        } 
        

        // std::vector<float> scene = dot.look(&walls, nullptr);   // We don't want to draw rays
        std::vector<float> scene = dot.look(walls, &window);      // We want to draw rays
        float w = WIDTH / scene.size();

        sf::Transform t;
        t.translate(WIDTH, 0);

        for(int idx = 0; auto s : scene) {
            // Farther away from wall means less brightness for single strip
            // Using inverse square law for better effect
            double sq = (double)(s*s);
            double wq = (double)(WIDTH*WIDTH);
            float brightness = hkk::map(sq, 0.0, wq, 255.0, 0.0);

            // float height = hkk::map((double)s, 0.0, (double)WIDTH, (double)HEIGHT, 0.0);
            // float height = hkk::map((double)s, 0.0, (double)WIDTH, (double)HEIGHT, (double)1/s);
            float lineH = (w*4000)/s;
            float lineOffset = 160 - lineH/2;

            // hkk::Rect r(idx*w + w/2, HEIGHT/2, w, height, hkk::Center);
            // r.fill(sf::Color(brightness, brightness, brightness, 255));
            // window.draw(r.rect, t);

            // Draw walls with thick lines
            sw::Line wall;
                wall.setThickness(w);
                wall.setPoint(wall.getStartIndex(), {idx*w+418, lineOffset});
                wall.setPoint(wall.getEndIndex()  , {idx*w+418, lineH+lineOffset});
                wall.setColor(sf::Color(brightness, brightness, brightness, 255));
            window.draw(wall);

            ++idx;
        } 
        
        // Move dot with mouse
        // dot.update(worldPos.x, worldPos.y);

        window.display();
    } return 0;
}       