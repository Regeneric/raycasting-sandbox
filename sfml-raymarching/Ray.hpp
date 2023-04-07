#pragma once 

#include <SFML/Graphics.hpp>

#include "Wall.hpp"

#include <vector>

class Ray {
public:
    Ray(sf::Vector2f p, float a);
    ~Ray() {}

    void draw(sf::RenderWindow *window);
    void look(std::vector<Wall> walls, sf::RenderWindow *window);
    float cast(std::vector<Wall> wall);

    inline void angle(float heading) {direction = hkk::fromAngle(heading);}
private:
    float _angle;
    sf::Vector2f position;
    sf::Vector2f direction;
};