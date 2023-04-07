#include "Boundry.hpp"
#include "commons.hpp"

#include <iostream>

void Boundry::draw(sf::RenderWindow *window, hkk::Shape _shape) {
    // // sf::Vertex line[2];
    // sf::VertexArray line(sf::Lines, 2);
    //     line[0].position = a;
    //     line[0].color = sf::Color::Red;

    //     line[1].position = b;
    //     line[1].color = sf::Color::Red;

    // // window->draw(line, 2, sf::Lines);
    // window->draw(line);

    // shape = _shape;
    // switch(shape) {
    //     case hkk::LineShape: {
    //         // hkk::Line l(a.x, a.y, b.x, b.y);

    //         hkk::Line l(a, b);
    //         window->draw(l.line);
    //     } break;
    //     case hkk::RectShape: {
    //         // hkk::Rect r(
    //         //     sf::Vector2f(10.0, 10.0), 
    //         //     sf::Vector2f(50.0, 10.0), 
    //         //     sf::Vector2f(50.0, 50.0),
    //         //     sf::Vector2f(10.0, 50.0));
    //         // window->draw(r.l1);
    //         // window->draw(r.l2);
    //         // window->draw(r.l3);
    //         // window->draw(r.l4);

    //         hkk::Rect r(a, b);
    //         window->draw(r.rect);
    //     } break;
    // }

    hkk::Line l(a, b);
    window->draw(l.line);
}