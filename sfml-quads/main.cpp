#include <SFML/Graphics.hpp>

#include "commons.hpp"
#include "Player.hpp"
#include "Wall.hpp"

#include <iostream>
#include <optional>


int main() {
    sf::RenderWindow window(sf::VideoMode(1024, HEIGHT), "Ray Casting", sf::Style::Close);
    window.setView(sf::View(sf::FloatRect(0, 0, 1024, HEIGHT)));


    std::vector<int> mapGrid {
        1,1,1,1,1,1,1,1,
        1,0,1,0,0,0,0,1,
        1,0,1,0,0,0,0,1,
        1,0,1,0,0,0,0,1,
        1,0,0,0,0,0,0,1,
        1,0,0,0,0,1,0,1,
        1,0,0,0,0,0,0,1,
        1,1,1,1,1,1,1,1,
    }; Wall map(8, 8, 64, mapGrid);
    
    Player player(sf::Vector2f(WIDTH/2, HEIGHT/2), sf::Vector2f(10.0f, 10.0f), std::nullopt);
    // Ray ray;


    while(window.isOpen()) {
        sf::Event e;
        while(window.pollEvent(e)) {
            switch(e.type) {case sf::Event::Closed: window.close(); break;}
        } window.clear(sf::Color(80, 80, 80));
        
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {player.rotate( 0.1f);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {player.rotate(-0.1f);}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {player.move( hkk::fromAngle(hkk::radians(player.rotation())));}
        if(sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {player.move(-hkk::fromAngle(hkk::radians(player.rotation())));}
        
        map.draw(&window);
        player.draw(&window);

        // player.look(&map, nullptr);  // Don't draw rays
        player.look(map, &window);     // Draw rays

        window.display();
    } return 0;
}