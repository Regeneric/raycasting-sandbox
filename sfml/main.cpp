#include <SFML/Graphics.hpp>

#include <iostream>
#include <vector>
#include <string>
#include <optional>
#include <random>

#include "Boundry.hpp"
#include "Ray.hpp"
#include "Particle.hpp"


static constexpr int WIDTH = 600;
static constexpr int HEIGHT = 600;


int main() {
    sf::RenderWindow window(sf::VideoMode(WIDTH, HEIGHT), "SFML Raycasting", sf::Style::Close);
        window.setView(sf::View(sf::FloatRect(0, 0, WIDTH, HEIGHT)));   // Viewport


    std::random_device rd;                            // Obtain a random number from hardware
    std::mt19937 gen(rd());                           // Seed the generator
    std::uniform_int_distribution<> rngW(0, WIDTH);   // Define the range
    std::uniform_int_distribution<> rngH(0, HEIGHT);  // Define the range


    std::vector<std::string> map {
        "##########",
        "#........#",
        "#........#",
        "#........#",
        "#........#",
        "#........#",
        "#........#",
        "#........#",
        "#........#",
        "##########"
    };

    std::vector<Boundry> walls;
    for(int i = 0; i != 5; ++i) {
        float x1 = rngW(gen);  float x2 = rngW(gen);
        float y1 = rngH(gen);  float y2 = rngW(gen);

        Boundry wall(x1, y1, x2, y2);
        walls.push_back(wall);
    }

    Boundry rectWall(10.0, 10.0, 100.0, 100.0);
    Particle dot(WIDTH/2, HEIGHT/2);

    std::vector<Boundry> rectWalls;
        rectWalls.push_back(rectWall);

    sf::Vector2i pixelPos;
    sf::Vector2f worldPos;

    while(window.isOpen()) {
        sf::Event e;
        while(window.pollEvent(e))
            switch(e.type) {case sf::Event::Closed: window.close(); break;}
        window.clear(sf::Color(80, 80, 80));

        pixelPos = sf::Mouse::getPosition(window);
        worldPos = window.mapPixelToCoords(pixelPos);

        dot.draw(&window);
        
        for(auto wall : walls) {
            wall.draw(&window, hkk::LineShape);
            rectWall.draw(&window, hkk::RectShape);
    
            // dot.look(&walls, nullptr);   // We don't want to draw rays
            dot.look(&walls, &window);      // We want to draw rays
            dot.look(&rectWalls, &window);
        } dot.update(worldPos.x, worldPos.y);

        window.display();
    } return 0;
}       