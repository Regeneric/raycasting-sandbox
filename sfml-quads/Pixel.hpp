#pragma once

#include <SFML/Graphics.hpp>

class Pixel {
public:
	Pixel(sf::Vector2f pos, float thick, sf::Color col);
	~Pixel() {}

	sf::Vertex verts[4];
	sf::VertexBuffer vertsBuff;
};