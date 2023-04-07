#pragma once

#include <SFML/Graphics.hpp>

#include "Ray.hpp"
#include "Boundry.hpp"

#include <vector>
#include <optional>

class Particle {
public:
    Particle(float x, float y, std::optional<int> angle, std::optional<int> fov);
    ~Particle() {}

    void draw(sf::RenderWindow *window);
    std::vector<float> look(std::vector<Boundry> *walls, sf::RenderWindow *window);
    void update(float x, float y);
private:
    sf::Vector2f position;
    std::vector<Ray> rays;
};