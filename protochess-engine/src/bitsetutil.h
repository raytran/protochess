//
// Created by raytran on 3/24/20.
//

#pragma once

#include <boost/dynamic_bitset.hpp>
#include "types.h"
#include "board.h"
#include <string>

//Bitboard utility functions
namespace protochess_engine {
    namespace bitsetUtil {
        boost::dynamic_bitset<> translate(Location delta, const boost::dynamic_bitset<> &in, const Board &board);

        boost::dynamic_bitset<>
        east(int amt, const boost::dynamic_bitset<> &in, const Board &board);

        boost::dynamic_bitset<>
        west(int amt, const boost::dynamic_bitset<> &in, const Board &board);

        boost::dynamic_bitset<>
        north(int amt, const boost::dynamic_bitset<> &in, const Board &board);

        boost::dynamic_bitset<>
        south(int amt, const boost::dynamic_bitset<> &in, const Board &board);

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

        Location getLoc(int width, int index);

        unsigned long findLast(const boost::dynamic_bitset<> &bitset);
    }
}
