#include "Ray.hpp"
#include "commons.hpp"

#include <vector>

void Ray::draw(sf::RenderWindow *window) {
    // sf::VertexArray line(sf::Lines, 2);
    //     line[0].position = sf::Vector2f(0.0, 0.0);
    //     line[0].color = sf::Color::Red;

    //     line[1].position = sf::Vector2f(direction.x * 10, direction.y * 10);
    //     line[1].color = sf::Color::Red;

    hkk::Line l(0.0, 0.0, direction.x*10, direction.y*10);
    
    sf::Transform t;
    t.translate(*position);

    window->draw(l.line, t);
}

std::optional<sf::Vector2f> Ray::cast(Boundry *wall, hkk::Shape shape) {
    auto wallPos = wall->position();

    // Wall start and end positions
    float x1 = wallPos[0].x;
    float y1 = wallPos[0].y;

    float x2 = 0.0;
    float y2 = 0.0;

    float area = 0.0;

    if(shape == hkk::LineShape) {
        x2 = wallPos[1].x;
        y2 = wallPos[1].y;
    }
    if(shape == hkk::RectShape) {
        x2 = wallPos[1].x + wallPos[0].x;
        y2 = wallPos[1].x + wallPos[0].y;

        area = x2 * y2;
    }

    // Ray position and directions
    float x3 = position->x;
    float y3 = position->y;

    float x4 = position->x + direction.x;
    float y4 = position->y + direction.y;

    // Naming of the variables comes from the Wikipedia page
    float den = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
    if(den == 0) return std::nullopt;  // Lines are parallel

    float t =  ((x1-x3)*(y3-y4) - (y1-y3)*(x3-x4)) / den;
    float u = -((x1-x2)*(y1-y3) - (y1-y2)*(x1-x3)) / den;

    if(t > 0 && t < 1  &&  u > 0) {
        sf::Vector2f point;
            point.x = x1 + t*(x2-x1);
            point.y = y1 + t*(y2-y1);
        return point;
    } return std::nullopt;  // Function may not return anything - same as `return {}`


    // if(shape == hkk::LineShape) {
    //     // Naming of the variables comes from the Wikipedia page
    //     float den = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
    //     if(den == 0) return std::nullopt;  // Lines are parallel

    //     float t =  ((x1-x3)*(y3-y4) - (y1-y3)*(x3-x4)) / den;
    //     float u = -((x1-x2)*(y1-y3) - (y1-y2)*(x1-x3)) / den;

    //     if(t > 0 && t < 1  &&  u > 0) {
    //         sf::Vector2f point;
    //             point.x = x1 + t*(x2-x1);
    //             point.y = y1 + t*(y2-y1);
    //         return point;
    //     } return std::nullopt;  // Function may not return anything - same as `return {}`
    // } else if(shape == hkk::RectShape) {
    //     // Naming of the variables comes from the Wikipedia page
    //     float den = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
    //     if(den == 0) return std::nullopt;  // Lines are parallel

    //     float t =  ((x1-x3)*(y3-y4) - (y1-y3)*(x3-x4)) / den;
    //     float u = -((x1-x2)*(y1-y3) - (y1-y2)*(x1-x3)) / den;

    //     if(t > 0 && t < 1  &&  u > 0) {
    //         sf::Vector2f point;
    //             point.x = x1 + t*(x2-x1);
    //             point.y = y1 + t*(y2-y1);
    //         return point;
    //     } return std::nullopt;  // Function may not return anything - same as `return {}`
    // } else return std::nullopt;
}


void Ray::look(float x, float y) {
    direction.x = x - position->x;
    direction.y = y - position->y;

    hkk::normalize(direction);
}