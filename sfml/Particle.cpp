#include "Particle.hpp"
#include "commons.hpp"

Particle::Particle(float x, float y, std::optional<int> angle) {
    position = sf::Vector2f(x, y);

    for(int a = 0; a != 360; a += (angle.has_value() ? angle.value() : 10)) {
        Ray ray(&position, hkk::radians(a));
        rays.push_back(ray);
    }
}

void Particle::draw(sf::RenderWindow *window) {
    sf::CircleShape pt;
        pt.setFillColor(sf::Color::Blue);
        pt.setRadius(8);
        pt.setOrigin(pt.getRadius(), pt.getRadius());
        pt.setPosition(position);

    window->draw(pt);
    for(auto ray : rays) ray.draw(window);
}

void Particle::look(std::vector<Boundry> *walls, sf::RenderWindow *window) {
    for(auto ray : rays) {
        sf::Vector2f closest(INFINITY, INFINITY);
        float max = INFINITY;

        for(auto wall : *walls) {
            std::optional<sf::Vector2f> point = ray.cast(&wall, hkk::LineShape);

            if(point.has_value()) {
                float dist = hkk::dist(position, point.value());

                // We only want cast rays to closest walls
                if(dist < max) {
                    max = dist;
                    closest = point.value();
                }
            }
        }

        if(closest != sf::Vector2f(INFINITY, INFINITY)) {
            hkk::Line l(position.x, position.y, closest.x, closest.y);
            if(window != nullptr) window->draw(l.line);
        }
    }
}

void Particle::update(float x, float y) {
    position.x = x;
    position.y = y;
}