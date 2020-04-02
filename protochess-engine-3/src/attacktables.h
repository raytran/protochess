//
// Created by raytran on 3/31/20.
//

#pragma once

#include <vector>
#include <map>
#include "types.h"
namespace protochess_engine{
    class AttackTables{
        bitboard zero = 0;
        bitboard boundaryMask = 0;
        int numBits = 0;
        std::vector<bitboard> northAttacks;
        std::vector<bitboard> southAttacks;
        std::vector<bitboard> eastAttacks;
        std::vector<bitboard> westAttacks;

        std::vector<bitboard> northEastAttacks;
        std::vector<bitboard> southEastAttacks;
        std::vector<bitboard> southWestAttacks;
        std::vector<bitboard> northWestAttacks;

        std::vector<bitboard> leftMasks;
        std::vector<bitboard> rightMasks;

        const Dimensions dimensions;

    public:

        explicit AttackTables(Dimensions dimensions);

        //Masks squares that are outside the game boundary
        //(Full board of 1s)
        const bitboard &getBoundaryMask() const;

        [[nodiscard]] const bitboard &getRayAttack(const Direction &dir, const int &index) const;

        [[nodiscard]] const bitboard &getRightMask(int numCols) const;

        [[nodiscard]] const bitboard &getLeftMask(int numCols) const;

        int getWidth() const;

        int getHeight() const;

        int getNumBits() const;

    };
}

