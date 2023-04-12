#include "Player.hpp"

Player::Player() : _rotation{0.0f}, _color{sf::Color::Red} {
    _player.setFillColor(_color);
}

Player::Player(sf::Vector2f p, sf::Vector2f s, std::optional<float> f) : Player() {
    _fov = f.has_value() ? f.value() : 60.0f;

    _size = s;
    _position = p;

    _player.setSize(_size);
    _player.setPosition(_position);
    _player.setOrigin(_size.x, _size.y/2);
}
Player::Player(sf::Vector2f p, sf::Vector2f s, sf::Color c, std::optional<float> f) : Player() {
    _fov = f.has_value() ? f.value() : 60.0f;

    _size = s;
    _position = p;
    _color = c;

    _player.setSize(_size);
    _player.setPosition(_position);
    _player.setFillColor(_color);
    _player.setOrigin(_size.x, _size.y/2);
}


void Player::rotate(float a) {
    _rotation += a;
    _player.rotate(a);
}

void Player::move(sf::Vector2f p, std::shared_ptr<Wall> map) {
    _position = p;
    std::vector<int> grid = map->grid();

    // Collision detecion with sliding
    int distFromWall = 12;
    float playerX = _player.getPosition().x;
    float playerY = _player.getPosition().y;

    int offsetX = 0;
    if(_position.x < 0) offsetX = -distFromWall;
    else offsetX = distFromWall;

    int offsetY = 0;
    if(_position.y < 0) offsetY = -distFromWall;
    else offsetY = distFromWall;

    int gridPosX = playerX/map->cell();
    int gridPosOffXAdd = (playerX + offsetX)/map->cell();

    int gridPosY = playerY/map->cell();
    int gridPosOffYAdd = (playerY + offsetY)/map->cell();


    if(grid[gridPosY * map->width()        + gridPosOffXAdd] == 0) {playerX += _position.x; _player.setPosition(playerX, playerY);}
    if(grid[gridPosOffYAdd * map->width()  + gridPosX]       == 0) {playerY += _position.y; _player.setPosition(playerX, playerY);}
}
void Player::move(float a, std::shared_ptr<Wall> map) {
    // sf::Vector2f fa = hkk::fromAngle(hkk::radians(a));
    // _rotation -= a;
    // _position = fa;

    // _player.move(_position);

    // TODO
}