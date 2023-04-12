#pragma once

#include <memory>

#include "Wall.hpp"

class Player;
class Ray {
public:
    Ray() {}
    ~Ray() {}

    void cast(float f, Player &player, std::shared_ptr<Wall> wall, std::shared_ptr<sf::RenderWindow> window);
};