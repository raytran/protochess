//
// Created by raytran on 3/24/20.
//

#pragma once

#include <string>
#include <map>
#include "types.h"
#include <boost/dynamic_bitset.hpp>

class Player {
private:
    std::string name;
    //MAPs char to a bitboard of that piece
    //Not using an enum here to allow for any arbitrary number of piece types
    std::map<char, boost::dynamic_bitset<>> pieces;
public:
    Player();

    explicit Player(std::string name);

    std::string getName();

    void setPieces(std::map<char, boost::dynamic_bitset<>> pieceMap);

    std::map<char, boost::dynamic_bitset<>> getPieces() const;
};


