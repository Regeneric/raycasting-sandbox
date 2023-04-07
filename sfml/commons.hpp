#pragma once

#include <SFML/Graphics.hpp>
#include <cmath>

namespace hkk {
    // Normalize vector
    static void normalize(sf::Vector2f &vec) {
        float mag = abs(sqrt(vec.x*vec.x + vec.y*vec.y));
        float magInv = 1.0/mag;

        vec.x *= magInv;
        vec.y *= magInv;
    }

    // Convert degrees to radians
    static inline float radians(float angle) {return angle * (M_PI/180.0f);}

    // Unit vector - heading based on angle in radians
    static inline sf::Vector2f fromAngle(float angle) {return sf::Vector2f(cos(angle), sin(angle));}

    // Distance between two points
    static inline float dist(sf::Vector2f a, sf::Vector2f b) {return sqrt(pow(a.x-b.x, 2) + pow(a.y-b.y, 2));}

    // Re-maps a number from one range to another.
    template<typename T>
    static inline T map(T number, T inMin, T inMax, T outMin, T outMax) {return (number-inMin) * (outMax-outMin)/(inMax-inMin) + outMin;}


    struct Line {
        sf::VertexArray line;
        sf::VertexArray getLine() {return line;}

        Line(sf::Vector2f startPos, sf::Vector2f endPos) {
            sf::Vertex start;
                start.position = startPos;
                start.color = sf::Color::White;

            sf::Vertex end;
                end.position = endPos;
                end.color = sf::Color::White;

            line.append(start);
            line.append(end);
            line.setPrimitiveType(sf::Lines);
        }
        Line(float x1, float y1, float x2, float y2) {
            sf::Vertex start;
                start.position = sf::Vector2f(x1, y1);
                start.color = sf::Color::White;

            sf::Vertex end;
                end.position = sf::Vector2f(x2, y2);
                end.color = sf::Color::White;

            line.append(start);
            line.append(end);
            line.setPrimitiveType(sf::Lines);
        }
    };

    struct Rect {
        sf::RectangleShape rect;

        Rect(float x1, float y1, float x2, float y2) {
            rect.setPosition(sf::Vector2f(x1, y1));
            rect.setSize(sf::Vector2f(x2, y2));
            rect.setFillColor(sf::Color::White);
        }
        Rect(sf::Vector2f pos, sf::Vector2f size) {
            rect.setPosition(pos);
            rect.setSize(size);
            rect.setFillColor(sf::Color::White);
        }

        void fill(sf::Color color) {rect.setFillColor(color);}

        // sf::VertexArray l1;
        // sf::VertexArray l2;
        // sf::VertexArray l3;
        // sf::VertexArray l4;

        // Rect(sf::Vector2f p1, sf::Vector2f p2, sf::Vector2f p3, sf::Vector2f p4) {
        //     hkk::Line e1(p1, p2);
        //     hkk::Line e2(p2, p3);
        //     hkk::Line e3(p3, p4);
        //     hkk::Line e4(p4, p1);

        //     l1 = e1.getLine();
        //     l2 = e2.getLine();
        //     l3 = e3.getLine();
        //     l4 = e4.getLine();
        // }
        // Rect(std::vector<sf::Vector2f> points) {
        //     hkk::Line e1(points[0], points[1]);
        //     hkk::Line e2(points[1], points[2]);
        //     hkk::Line e3(points[2], points[3]);
        //     hkk::Line e4(points[3], points[0]);
        // }
    };
}