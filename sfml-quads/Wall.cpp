#include <SFML/Graphics.hpp>

#include "Wall.hpp"

Wall::Wall(int w, int h, int c, std::vector<int> m) {
    _width = w;
    _height = h;
    _cell = c;
    _grid = m;
}


void Wall::draw(sf::RenderWindow *window) {
    for(int y = 0; y < _height; y++) {
        for(int x = 0; x < _width; x++) {
            sf::RectangleShape wall;
            if(_grid[y * _width + x] == 1) wall.setFillColor(sf::Color::Black);
            else wall.setFillColor(sf::Color::White);

            int xs = x * _cell;
            int ys = y * _cell;

            wall.setSize(sf::Vector2f(_cell-1, _cell-1));
            wall.setPosition(sf::Vector2f(xs, ys));
            
            window->draw(wall);   
        }
    }
}