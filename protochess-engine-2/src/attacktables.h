//
// Created by raytran on 3/31/20.
//

#pragma once

#include <vector>
#include <map>
#include <boost/dynamic_bitset.hpp>
#include "types.h"
using boost::dynamic_bitset;
namespace protochess_engine{
    class AttackTables{
        std::vector<dynamic_bitset<>> northAttacks;
        std::vector<dynamic_bitset<>> southAttacks;
        std::vector<dynamic_bitset<>> eastAttacks;
        std::vector<dynamic_bitset<>> westAttacks;

        std::vector<dynamic_bitset<>> northEastAttacks;
        std::vector<dynamic_bitset<>> southEastAttacks;
        std::vector<dynamic_bitset<>> southWestAttacks;
        std::vector<dynamic_bitset<>> northWestAttacks;

        std::vector<dynamic_bitset<>> leftMasks;
        std::vector<dynamic_bitset<>> rightMasks;

        const Dimensions dimensions;

    public:
        explicit AttackTables(Dimensions dimensions);

        [[nodiscard]] const dynamic_bitset<> &getRayAttack(const Direction &dir, const int &index) const;

        dynamic_bitset<> getRightMask(int numCols) const;

        dynamic_bitset<> getLeftMask(int numCols) const;

        int getWidth() const;

        int getHeight() const;
    };
}
