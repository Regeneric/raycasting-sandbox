#pragma once

#include <SFML/Graphics.hpp>

#include <algorithm>
#include <optional>
#include <cmath>

static constexpr int WIDTH  = 512;
static constexpr int HEIGHT = 512;

// static constexpr int WIDTH  = 720;
// static constexpr int HEIGHT = 720;

namespace hkk {
    enum Shape {
        Circle,
        Square
    };

    // Distance between two points
    static inline float dist(float x1, float y1, float x2, float y2) {return sqrt(pow(x1-x2, 2) + pow(y1-y2, 2));}
    static inline float dist(sf::Vector2f a, sf::Vector2f b) {return sqrt(pow(a.x-b.x, 2) + pow(a.y-b.y, 2));}
    static inline float dist(sf::Vector2f v) {return sqrt(v.x*v.x + v.y+v.y);}  // P -- C  ; v = C - P

    // Distance between point and circle or square centre
    static inline float signedDistanceCircle(sf::Vector2f p, sf::Vector2f c, float r) {return dist(c-p) - r;}
    static inline float signedDistanceSquare(sf::Vector2f p, sf::Vector2f c, sf::Vector2f s) {
        sf::Vector2f offset;
            offset.x = abs(p.x - c.x) - s.x;
            offset.y = abs(p.y - c.y) - s.y;

        // Distance from point outside the box to the edge (0 if inside)
        float unsignedDist = dist(sf::Vector2f(std::max(offset.x, 0.0f), std::max(offset.y, 0.0f))); 

        // Negative distance from point inside the box to the edge (0 if outside)
        float signedDist = std::max(std::min(offset.x, 0.0f), std::min(offset.y, 0.0f));

        return unsignedDist + signedDist;
    }


    // Normalize vector
    static void normalize(sf::Vector2f &vec) {
        float mag = abs(sqrt(vec.x*vec.x + vec.y*vec.y));
        float magInv = 1.0/mag;

        vec.x *= magInv;
        vec.y *= magInv;
    }

    // Set vector magnitude
    static void mag(sf::Vector2f &vec, float m) {
        vec.x *= m;
        vec.y *= m;
    }

    // Convert degrees to radians
    static inline constexpr float radians(float angle) {return angle * (M_PI/180.0f);}

    // Convert radians to degrees
    static inline constexpr float degrees(float angle) {return angle * 180.f/M_PI;}

    // Unit vector - heading based on angle in radians
    static inline sf::Vector2f fromAngle(float angle) {return sf::Vector2f(cos(angle)/15, sin(angle)/15);}

    // Re-maps a number from one range to another.
    template<typename T>
    static inline constexpr T map(T number, T inMin, T inMax, T outMin, T outMax) {return (number-inMin) * (outMax-outMin)/(inMax-inMin) + outMin;}

    // Get angle from unit vector
    static inline double heading(sf::Vector2f vec) {return atan2(vec.y, vec.x);}

    // Clockwise, perpendicular vector to oter vector
    static inline sf::Vector2f perpendicular(sf::Vector2f fromVector) {return sf::Vector2f(-fromVector.y, fromVector.x);}


    struct Line {
        sf::VertexArray line;
        sf::VertexArray getLine() {return line;}
        void fill(sf::Color c) {line[0].color = c; line[0].color = c;}

        Line(sf::Vector2f startPos, sf::Vector2f endPos) {
            sf::Vertex start;
                start.position = startPos;
                start.color = sf::Color::Red;

            sf::Vertex end;
                end.position = endPos;
                end.color = sf::Color::Red;

            line.append(start);
            line.append(end);
            line.setPrimitiveType(sf::Lines);
        }
        Line(float x1, float y1, float x2, float y2) {
            sf::Vertex start;
                start.position = sf::Vector2f(x1, y1);
                start.color = sf::Color::Red;

            sf::Vertex end;
                end.position = sf::Vector2f(x2, y2);
                end.color = sf::Color::Red;

            line.append(start);
            line.append(end);
            line.setPrimitiveType(sf::Lines);
        }
    };

    enum RectMode {
        Center,
        Normal
    };

    struct Rect {
        sf::RectangleShape rect;

        Rect(float x1, float y1, float x2, float y2, std::optional<RectMode> mode) {
            rect.setPosition(sf::Vector2f(x1, y1));
            rect.setSize(sf::Vector2f(x2, y2));
            rect.setFillColor(sf::Color::White);

            if(mode.has_value() && mode.value() == RectMode::Center) rect.setOrigin(x2/2, y2/2);
            if(mode.has_value() && mode.value() == RectMode::Normal) rect.setOrigin(0, 0);
        }
        Rect(sf::Vector2f pos, sf::Vector2f size, std::optional<RectMode> mode) {
            rect.setPosition(pos);
            rect.setSize(size);
            rect.setFillColor(sf::Color::White);

            if(mode.has_value() && mode.value() == RectMode::Center) rect.setOrigin(size.x/2, size.y/2);
            if(mode.has_value() && mode.value() == RectMode::Normal) rect.setOrigin(0, 0);
        }

        void fill(sf::Color color) {rect.setFillColor(color);}
    };
}