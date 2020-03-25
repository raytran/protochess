//
// Created by raytran on 3/24/20.
//

#pragma once

#include <boost/dynamic_bitset.hpp>
#include "types.h"
#include "Board.h"
#include <string>

//Bitboard utility functions
namespace bitsetUtil {
    boost::dynamic_bitset<>
    eastOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    westOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    northOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    southOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    northEastOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    northWestOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    southEastOne(const boost::dynamic_bitset<> &in, const Board &board);

    boost::dynamic_bitset<>
    southWestOne(const boost::dynamic_bitset<> &in, const Board &board);

    std::string bitsetToString(const boost::dynamic_bitset<> &bitset, const Dimensions &dimensions);

    int getIndex(int width, Location loc);
}


