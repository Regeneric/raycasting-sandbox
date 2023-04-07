#pragma once

#include "commons.hpp"

#include <optional>

class Wall {
public:
    Wall(sf::Vector2f p, sf::Vector2f s, std::optional<float> r, hkk::Shape t)
        : _position{p}, _size{s}, _radius{r.has_value() ? r.value() : 0.0f}, _type{t} {} 
    ~Wall() {}

    void draw(sf::RenderWindow *window, hkk::Shape shape);

    hkk::Shape type();
    sf::Vector2f position();
    sf::Vector2f size();
    float radius();
private:
    hkk::Shape _type;
    
    sf::Vector2f _position;
    sf::Vector2f _size;
    
    float _radius;
};