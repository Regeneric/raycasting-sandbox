#include "Pixel.hpp"

Pixel::Pixel(sf::Vector2f pos, float thick, sf::Color col) {
    verts[0].position = pos;
    verts[1].position = sf::Vector2f(pos.x + thick, pos.y);
    verts[2].position = sf::Vector2f(pos.x + thick, pos.y + thick);
    verts[3].position = sf::Vector2f(pos.x, pos.y + thick);


    for(auto v : verts) v.color = col;
    vertsBuff.update(verts);
}