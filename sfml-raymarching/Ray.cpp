#include "Ray.hpp"
#include "commons.hpp"

#include <cmath>

Ray::Ray(sf::Vector2f p, float a) {
    position = p;
    direction = hkk::fromAngle(a);
}

float Ray::cast(std::vector<Wall> walls) {
    float sceneDist = INFINITY;
    
    for(auto wall : walls) {
        if(wall.type() == hkk::Circle) {
            float circleDist = hkk::signedDistanceCircle(position, wall.position(), wall.radius());
            sceneDist = std::min(circleDist, sceneDist); 
        }

        if(wall.type() == hkk::Square) {
            float squareDist = hkk::signedDistanceSquare(position, wall.position(), wall.size());
            sceneDist = std::min(squareDist, sceneDist);
        }
    } return sceneDist;
}

// void Ray::look(std::vector<Wall> walls, sf::RenderWindow *window) {

// }

void Ray::draw(sf::RenderWindow *window) {
    sf::Transform t;
    t.translate(position);

    window->draw(hkk::Line(position, hkk::fromAngle(_angle)).line, t);
}