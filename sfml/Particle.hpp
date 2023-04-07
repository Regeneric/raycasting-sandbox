#pragma once

#include <SFML/Graphics.hpp>

#include "Ray.hpp"
#include "Boundry.hpp"

#include <vector>
#include <optional>

class Particle {
public:
    Particle(float x, float y, std::optional<int> angle);
    ~Particle() {}

    void draw(sf::RenderWindow *window);
    void look(std::vector<Boundry> *walls, sf::RenderWindow *window);
    void update(float x, float y);
private:
    sf::Vector2f position;
    std::vector<Ray> rays;
};