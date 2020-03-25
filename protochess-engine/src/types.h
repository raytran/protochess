//
// Created by raytran on 3/24/20.
//

#pragma once

#include <vector>

struct Dimensions {
    int width;
    int height;
};

struct Location {
    int x;
    int y;
};

struct MovementPattern {
    bool north;
    bool east;
    bool south;
    bool west;
    bool northEast;
    bool northWest;
    bool southEast;
    bool southWest;
    std::vector<Location> deltas;
};

struct Move {
    Location start;
    Location end;
};

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST
};
