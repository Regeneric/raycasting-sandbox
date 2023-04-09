#pragma once

#include <vector>

class Wall {
public:
    Wall(int w, int h, int c, std::vector<int> m);
    ~Wall() {}

    void draw(sf::RenderWindow *window);

    void cell(int c)   {_cell = c;} 
    constexpr int cell()   {return _cell;}

    void width(int w)  {_width = w;}
    constexpr int width()  {return _width;}

    void height(int h) {_height = h;}
    constexpr int height() {return _height;}

    void grid(std::vector<int> g) {_grid = g;}
    constexpr std::vector<int> grid() {return _grid;}

    std::vector<sf::FloatRect> bounds() {return _wallsBounds;}

private:
    int _cell;
    int _width;
    int _height;

    std::vector<int> _grid;
    std::vector<sf::FloatRect> _wallsBounds; 

    void initBounds();
};