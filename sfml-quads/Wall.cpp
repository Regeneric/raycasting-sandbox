#include <SFML/Graphics.hpp>

#include "Wall.hpp"

Wall::Wall(int w, int h, int c, std::vector<int> m) {
    _width = w;
    _height = h;
    _cell = c;
    _grid = m;

    initBounds();
}


void Wall::draw(sf::RenderWindow *window) {
    for(int y = 0; y < _height; y++) {
        for(int x = 0; x < _width; x++) {
            sf::RectangleShape wall;
            if(_grid[y * _width + x] > 0) wall.setFillColor(sf::Color::Black);
            else wall.setFillColor(sf::Color::White);

            int xs = x * _cell;
            int ys = y * _cell;

            wall.setSize(sf::Vector2f(_cell-1, _cell-1));
            wall.setPosition(sf::Vector2f(xs, ys));
            
            window->draw(wall);   
        }
    }
}

void Wall::initBounds() {
    for(int y = 0; y < _height; y++) {
        for(int x = 0; x < _width; x++) {
            int xs = x * _cell;
            int ys = y * _cell;

            sf::RectangleShape wall;
                wall.setSize(sf::Vector2f(_cell-1, _cell-1));
                wall.setPosition(sf::Vector2f(xs, ys)); 

            if(_grid[y * _width + x] > 0) {
                _wallsBounds.push_back(wall.getGlobalBounds());
            } else wall.setFillColor(sf::Color::White); 
        }
    }
}