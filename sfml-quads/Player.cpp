#include "Player.hpp"

Player::Player() : _rotation{0.0f}, _shape{hkk::Square}, _color{sf::Color::Red} {
    _player.setFillColor(_color);
}

Player::Player(sf::Vector2f p, sf::Vector2f s, std::optional<float> f) : Player() {
    _fov = f.has_value() ? f.value() : 60.0f;

    _size = s;
    _position = p;

    _player.setSize(_size);
    _player.setPosition(_position);
    _player.setOrigin(_size.x/2, _size.y/2);
}
Player::Player(sf::Vector2f p, sf::Vector2f s, sf::Color c, std::optional<float> f) : Player() {
    _fov = f.has_value() ? f.value() : 60.0f;

    _size = s;
    _position = p;
    _color = c;

    _player.setSize(_size);
    _player.setPosition(_position);
    _player.setFillColor(_color);
    _player.setOrigin(_size.x/2, _size.y/2);
}


void Player::rotate(float a) {
    _rotation += a;
    _player.rotate(a);
}

void Player::move(sf::Vector2f p) {
    _position = p;
    _player.move(_position);
}
void Player::move(float a) {
    sf::Vector2f fa = hkk::fromAngle(hkk::radians(a));
    _rotation -= a;
    _position = fa;

    _player.move(_position);
}


void Player::look(Wall map, sf::RenderWindow *window) {
    _ray.cast(_fov, *this, map, window);
}