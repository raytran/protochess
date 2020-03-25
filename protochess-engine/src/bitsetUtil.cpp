//
// Created by raytran on 3/24/20.
//

#include "bitsetUtil.h"

namespace bitsetUtil {
    boost::dynamic_bitset<>
    northEastOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in << (board.getWidth() + 1)) & ~board.getLeftMostFile();
    }

    boost::dynamic_bitset<>
    northWestOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in << (board.getWidth() - 1)) & ~board.getRightMostFile();
    }

    boost::dynamic_bitset<>
    southEastOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in >> (board.getWidth() - 1)) & ~board.getLeftMostFile();

    }

    boost::dynamic_bitset<>
    southWestOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in >> (board.getWidth() + 1)) & ~board.getRightMostFile();
    }

    boost::dynamic_bitset<>
    eastOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in << 1) & ~board.getLeftMostFile();
    }

    boost::dynamic_bitset<>
    westOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return (in >> 1) & ~board.getRightMostFile();
    }

    boost::dynamic_bitset<>
    northOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return in << board.getWidth();
    }

    boost::dynamic_bitset<>
    southOne(const boost::dynamic_bitset<> &in, const Board &board) {
        return in >> board.getWidth();
    }

    std::string bitsetToString(const boost::dynamic_bitset<> &bitboard, const Dimensions &dimensions) {
        std::string returnString = "";
        int width = dimensions.width;
        int height = dimensions.height;
        for (int y = height - 1; y >= 0; y--) {
            for (int x = 0; x < width; x++) {
                if (bitboard[getIndex(width, {x, y})]) {
                    returnString += "1 ";
                } else {
                    returnString += ". ";
                }
            }
            returnString += "\n";
        }
        return returnString;
    }

    int getIndex(int width, Location loc) {
        return loc.y * width + loc.x;
    }

}

