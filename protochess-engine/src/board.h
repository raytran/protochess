//
// Created by raytran on 3/25/20.
//

#pragma once


#include <boost/dynamic_bitset.hpp>
#include "player.h"

namespace protochess_engine {
    using boost::dynamic_bitset;

    class Board {
    private:
        Dimensions dimensions;
        dynamic_bitset<> allPieces;
        dynamic_bitset<> leftMostFile;
        dynamic_bitset<> rightMostFile;
        std::vector<Tile> tiles;
        std::map<Direction, std::vector<dynamic_bitset<>>> rayAttacks;

        /*
        Contains file masks (0 indexed) s.t.
            leftMasks[2] =
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0
         1 1 1 0 0 0 0 0


            rightMasks[2] =
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         0 0 0 0 0 1 1 1
         Useful for east-west bit-shifting
         */


        std::vector<dynamic_bitset<>> leftMasks;
        std::vector<dynamic_bitset<>> rightMasks;
    public:
        Board(int width, int height);

        void update(std::map<int, Player> &players);

        int getWidth() const;

        int getHeight() const;

        std::vector<Tile> &getTiles();

        dynamic_bitset<> getRayAttack(const Direction &dir, const int &index);

        dynamic_bitset<> getRightMostFile() const;

        dynamic_bitset<> getLeftMostFile() const;

        dynamic_bitset<> getAllPieces() const;

        dynamic_bitset<> getRightMask(int numCols) const;

        dynamic_bitset<> getLeftMask(int numCols) const;

        Dimensions getDimensions() const;

    };
}
