//
// Created by raytran on 3/31/20.
//
#pragma once
#include <boost/dynamic_bitset.hpp>
#include "types.h"
#include "attacktables.h"
#include <string>

//Bitboard utility functions
namespace protochess_engine::bitsetUtil {
    using boost::dynamic_bitset;

    std::string bitsetToString(const dynamic_bitset<> &bitset, const Dimensions &dimensions);

    dynamic_bitset<> toBitset(const Location &loc, const Dimensions &dimensions);

    dynamic_bitset<> translate(Location delta, const dynamic_bitset<> &in, const AttackTables &attackTables);

    long findLast(const dynamic_bitset<> &bitset);

    int getIndex(int width, Location loc);

    Location getLoc(int width, int index);

    long findLast(const dynamic_bitset<> &bitset);
}


