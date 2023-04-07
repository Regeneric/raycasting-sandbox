#pragma once

#include <SFML/Graphics.hpp>

#include "Boundry.hpp"
#include "commons.hpp"

#include <iostream>
#include <optional>
#include <cmath>

class Ray {
public:
    Ray(sf::Vector2f *p, float a)
        : position{p}, direction{hkk::fromAngle(a)}, angle{a} {}
    ~Ray() {}

    void draw(sf::RenderWindow *window);
    std::optional<sf::Vector2f> cast(Boundry *wall, hkk::Shape shape);
    void look(float x, float y);
    inline void setAngle(float heading) {direction = hkk::fromAngle(heading);}

    sf::Vector2f getDirection() {return direction;}

private:
    float angle;
    sf::Vector2f *position;
    sf::Vector2f direction;     // Unit vector - heading based on angle in radians
};