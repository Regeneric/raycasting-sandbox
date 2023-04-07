#pragma once

#include <SFML/Graphics.hpp>

#include <vector>

namespace hkk {
    enum Shape {
        RectShape,
        LineShape
    };
}

class Boundry {
public:
    hkk::Shape shape;

    Boundry(float x1, float y1, float x2, float y2) 
        : a{sf::Vector2f(x1, y1)}, b{sf::Vector2f(x2, y2)},
          pos{sf::Vector2f(x1, y2)}, size{sf::Vector2f(x2, y2)} {}
    ~Boundry() {}

    void draw(sf::RenderWindow *window, hkk::Shape _shape);
    std::vector<sf::Vector2f> position() {return std::vector<sf::Vector2f> {a, b};}
private:
    sf::Vector2f a;
    sf::Vector2f b;

    sf::Vector2f pos;
    sf::Vector2f size;
};