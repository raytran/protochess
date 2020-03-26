//
// Created by raytran on 3/25/20.
//

#pragma once

#include <boost/uuid/uuid.hpp>
#include <boost/dynamic_bitset.hpp>
#include "types.h"

class Piece {
private:
    char charRep;
    boost::uuids::uuid id;
    boost::dynamic_bitset<> bitset;
    Location location;
    int locationIndex;
public:
    Piece(boost::uuids::uuid id, boost::dynamic_bitset<> bitset, char charRep, Location loc, int locationIndex);

    boost::dynamic_bitset<> getBitset() const;

    char getCharRep() const;

    void setCharRep(char c);

    void setBitset(boost::dynamic_bitset<> bitset);

    Location getLocation();

    int getLocationIndex() const;

    void setLocation(Location loc, int index);
};


