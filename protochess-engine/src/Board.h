//
// Created by raytran on 3/25/20.
//

#pragma once


#include <boost/dynamic_bitset.hpp>
#include "player.h"

namespace protochess_engine {
    class Board {
    private:
        Dimensions dimensions;
        boost::dynamic_bitset<> allPieces;
        boost::dynamic_bitset<> leftMostFile;
        boost::dynamic_bitset<> rightMostFile;
        std::map<Direction, std::vector<boost::dynamic_bitset<>>> rayAttacks;

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


        std::vector<boost::dynamic_bitset<>> leftMasks;
        std::vector<boost::dynamic_bitset<>> rightMasks;
    public:
        Board(int width, int height);

        void update(std::map<int, Player> &players);

        int getWidth() const;

        int getHeight() const;

        boost::dynamic_bitset<> getRayAttack(const Direction &dir, const int &index);

        boost::dynamic_bitset<> getRightMostFile() const;

        boost::dynamic_bitset<> getLeftMostFile() const;

        boost::dynamic_bitset<> getAllPieces() const;

        boost::dynamic_bitset<> getRightMask(int numCols) const;

        boost::dynamic_bitset<> getLeftMask(int numCols) const;

        Dimensions getDimensions() const;
    };
}
