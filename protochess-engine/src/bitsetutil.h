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
        using boost::dynamic_bitset;

        dynamic_bitset<> translate(Location delta, const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        east(int amt, const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        west(int amt, const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        north(int amt, const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        south(int amt, const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        northEastOne(const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        northWestOne(const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        southEastOne(const dynamic_bitset<> &in, const Board &board);

        dynamic_bitset<>
        southWestOne(const dynamic_bitset<> &in, const Board &board);

        std::string bitsetToString(const dynamic_bitset<> &bitset, const Dimensions &dimensions);

        int getIndex(int width, Location loc);

        Location getLoc(int width, int index);

        long findLast(const dynamic_bitset<> &bitset);
    }
}
