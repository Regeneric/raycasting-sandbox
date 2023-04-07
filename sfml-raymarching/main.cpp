#include <SFML/Graphics.hpp>

#include "Ray.hpp"
#include "Wall.hpp"
#include "commons.hpp"

#include <iostream>
#include <optional>

static constexpr int WIDTH  = 800;
static constexpr int HEIGHT = 800;

int main() {
    sf::RenderWindow window(sf::VideoMode(WIDTH, HEIGHT), "Ray Marching", sf::Style::Close);
    window.setView(sf::View(sf::FloatRect(0, 0, WIDTH, HEIGHT)));

    Ray ray(sf::Vector2f(100.0f, 100.0f), hkk::radians(0));
    
    std::vector<Wall> walls;
    Wall wall(sf::Vector2f(200.0f, 100.0f), sf::Vector2f(10.0f, 100.0f), std::nullopt, hkk::Square);
    walls.push_back(wall);

    while(window.isOpen()) {
        sf::Event e;
        while(window.pollEvent(e)) {
            switch(e.type) {case sf::Event::Closed: window.close(); break;}
        } window.clear(sf::Color(80, 80, 80));
        
        ray.draw(&window);
        wall.draw(&window, hkk::Square);

        std::cout << ray.cast(walls) << std::endl;

        window.display();
    } return 0;
}