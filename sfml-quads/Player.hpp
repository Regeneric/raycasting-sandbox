#pragma once

#include <SFML/Graphics.hpp>

#include "commons.hpp"
#include "Wall.hpp"
#include "Ray.hpp"

#include <any>
#include <optional>

class Player {
public:
    Player();
    Player(sf::Vector2f p, sf::Vector2f s, std::optional<float> f);
    Player(sf::Vector2f p, sf::Vector2f s, sf::Color c, std::optional<float> f);
    ~Player() {}


    void fov(float f) {_fov = f;}
    constexpr float fov() {return _fov;}

    void rotate(float a);
    void move(float a);
    void move(sf::Vector2f p);

    void draw(sf::RenderWindow *window) {window->draw(_player);};
    void look(Wall map, sf::RenderWindow *window);


    void color(sf::Color c) {_color = c;}
    sf::Color color() {return _color;}

    void size(sf::Vector2f s) {_size = s;}
    sf::Vector2f size() {return _size;}

    void position(sf::Vector2f p) {_position = p;}
    sf::Vector2f position() {return _player.getPosition();}

    void shape(hkk::Shape s) {_shape = s;}
    hkk::Shape shape() {return _shape;}

    constexpr float rotation() {return _rotation;}

private:
    float _fov;
    sf::RectangleShape _player;

    sf::Color _color;

    float _rotation;
    sf::Vector2f _size;
    sf::Vector2f _position;

    hkk::Shape _shape;

    Ray _ray;
};