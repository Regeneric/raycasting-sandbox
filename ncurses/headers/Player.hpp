#pragma once 

#include <string>
#include <cmath>

struct Screen {
    int width;
    int height;
};

using std::string;
class Player {
public:
    Player() 
        : posX{0.0}, posY{0.0}, 
          rotAngle{0.0}, fov{M_PI/4.0},
          drawDistance{16.0} {} 

    Player(float x, float y)
        : posX{x}, posY{y},
          rotAngle(0.0), fov(M_PI/4.0),
          drawDistance(16.0) {}

    Player(float f, float dd) 
        : fov{f}, drawDistance{dd} {}

    Player(float x, float y, float f, float dd)
        : posX{x}, posY{y}, rotAngle{0.0}, 
          fov{f}, drawDistance{dd} {}
    
    ~Player() {}


    void move(char c, float delta);
    void raycast(const Screen *screen);

private:
    float posX;
    float posY;
    float rotAngle;

    float fov;
    float drawDistance;
};