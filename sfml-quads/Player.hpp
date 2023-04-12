#pragma once

#include <SFML/Graphics.hpp>

#include "commons.hpp"
#include "Wall.hpp"
#include "Ray.hpp"

#include <optional>
#include <memory>

class Player {
public:
    Player();
    Player(sf::Vector2f p, sf::Vector2f s, std::optional<float> f);
    Player(sf::Vector2f p, sf::Vector2f s, sf::Color c, std::optional<float> f);
    ~Player() {}


    void fov(float f) {_fov = f;}
    constexpr float fov() {return _fov;}

    void rotate(float a);
    void move(float a, std::shared_ptr<Wall> map);
    void move(sf::Vector2f p, std::shared_ptr<Wall> map);

    void draw(std::shared_ptr<sf::RenderWindow> window) {window->draw(_player);};
    void look(std::shared_ptr<Wall> map, std::shared_ptr<sf::RenderWindow> window) {_ray.cast(_fov, *this, map, window);}


    void color(sf::Color c) {_color = c;}
    sf::Color color() {return _color;}

    void size(sf::Vector2f s) {_size = s;}
    sf::Vector2f size() {return _size;}

    void position(sf::Vector2f p) {_position = p;}
    sf::Vector2f position() {return _player.getPosition();}

    constexpr float rotation() {return _rotation;}

private:
    float _fov;
    sf::RectangleShape _player;

    sf::Color _color;

    float _rotation;
    sf::Vector2f _size;
    sf::Vector2f _position;

    Ray _ray;
};