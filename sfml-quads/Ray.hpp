#pragma once

// #include "Player.hpp"
#include "Wall.hpp"

class Player;
class Ray {
public:
    Ray() {}
    ~Ray() {}

    void cast(float f, Player &player, Wall wall, sf::RenderWindow *window);
};