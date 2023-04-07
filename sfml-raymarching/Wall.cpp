#include "Wall.hpp"

hkk::Shape Wall::type() {return _type;}

sf::Vector2f Wall::position() {return _position;}
sf::Vector2f Wall::size() {return _size;}

float Wall::radius() {return _radius;}

void Wall::draw(sf::RenderWindow *window, hkk::Shape shape) {
    // sf::Transform t;
    // t.translate(position);

    // window->draw(hkk::Line(0.0f, 0.0f, 10.0f, 0.0f).line, t);

    if(shape == hkk::Circle) {}
    if(shape == hkk::Square) {
        window->draw(hkk::Rect(_position, _size, hkk::Center).rect);
    }
}