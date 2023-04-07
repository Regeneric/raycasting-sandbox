#include "Particle.hpp"
#include "commons.hpp"

#include <iostream>

Particle::Particle(float x, float y, std::optional<float> an, std::optional<int> f) {
    heading = 0.0;
    position = sf::Vector2f(x, y);
    fov = f.has_value() ? f.value() : 50;
    angle = an.has_value() ? an.value() : 10;

    for(float a = -(fov/2);  a < (fov/2);  a += angle) {
        Ray ray(&position, hkk::radians(a));
        rays.push_back(ray);
    }

    // for(int a = 0;  a < 40;  a += 1) {
    //     Ray ray(&position, hkk::radians(a));
    //     rays.push_back(ray);
    // }
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

std::vector<float> Particle::look(std::vector<Boundry> walls, sf::RenderWindow *window) {
    std::vector<float> scene;

    for(auto &ray : rays) {
        sf::Vector2f closest(INFINITY, INFINITY);
        float max = INFINITY;

        for(int idx = 0; auto wall : walls) {
            std::optional<sf::Vector2f> point = ray.cast(&wall, hkk::LineShape);

            if(point.has_value()) {
                float dist = hkk::dist(position, point.value());
                
                // float rd = sqrt((ray.getDirection().x * ray.getDirection().y) + (ray.getDirection().y*ray.getDirection().y) * cos(fov/2 + ((float)fov/rays.size()) * hkk::radians(idx)));
                // dist /= rd;
                
                float ang = hkk::heading(ray.getDirection()) - heading;
                dist *= cos(ang);

                // We only want cast rays to closest walls
                if(dist < max) {
                    max = dist;
                    closest = point.value();
                }
            } idx++;
        }

        if(closest != sf::Vector2f(INFINITY, INFINITY)) {
            hkk::Line l(position.x, position.y, closest.x, closest.y);
            if(window != nullptr) window->draw(l.line);
        } scene.push_back(max);
    } return scene;
}

void Particle::update(float x, float y) {
    position.x = x;
    position.y = y;
}


void Particle::rotate(float a) {
    heading += a;
    int index = 0;

    // for(float idx = 0; auto &ray : rays) {
    //     ray.setAngle(hkk::radians(idx) + heading);
    //     idx += angle;
    // }

    for(float i = -(fov/2); i < (fov/2); i += angle) {
        rays[index].setAngle(hkk::radians(i) + heading);
        index++;
    }
}

void Particle::move(float m) {
    sf::Vector2f vel = hkk::fromAngle(heading);
    hkk::mag(vel, m);
    
    position.x += vel.x;
    position.y += vel.y;
}