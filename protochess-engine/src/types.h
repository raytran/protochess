//
// Created by raytran on 3/24/20.
//

#pragma once

#include <vector>
#include <boost/uuid/uuid.hpp>
#include <boost/dynamic_bitset.hpp>

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

struct LocationDelta {
    Location start;
    Location end;
};

struct Move {
    bool capture;
    LocationDelta locationDelta;
};

typedef boost::dynamic_bitset<>::size_type size_type;

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
