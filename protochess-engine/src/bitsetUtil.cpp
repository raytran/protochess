//
// Created by raytran on 3/24/20.
//

#include <iostream>
#include "bitsetUtil.h"

namespace protochess_engine {
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
        east(int amt, const boost::dynamic_bitset<> &in, const Board &board) {
            return (in << amt) & ~board.getLeftMask(amt);
        }

        boost::dynamic_bitset<>
        west(int amt, const boost::dynamic_bitset<> &in, const Board &board) {
            return (in >> amt) & ~board.getRightMask(amt);
        }

        boost::dynamic_bitset<>
        north(int amt, const boost::dynamic_bitset<> &in, const Board &board) {
            return in << (amt * board.getWidth());
        }

        boost::dynamic_bitset<>
        south(int amt, const boost::dynamic_bitset<> &in, const Board &board) {
            return in >> (amt * board.getWidth());
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

        Location getLoc(int width, int index) {
            return {index % width, index / width};
        }

        //Like find_first, but for last
        unsigned long findLast(const boost::dynamic_bitset<> &bitset) {
            unsigned long index = -1;
            for (unsigned long i = bitset.size() - 1; i >= 0; i--) {
                if (bitset[i]) {
                    index = i;
                    break;
                }
            }
            return index;
        }

        boost::dynamic_bitset<> translate(Location delta, const boost::dynamic_bitset<> &in, const Board &board) {
            boost::dynamic_bitset<> returnBits(in);
            int dx = delta.x;
            int dy = delta.y;
            if (dx > 0) {
                returnBits = east(dx, returnBits, board);
            } else {
                returnBits = west(-dx, returnBits, board);
            }
            if (dy > 0) {
                returnBits = north(dy, returnBits, board);
            } else {
                returnBits = south(-dy, returnBits, board);
            }
            return returnBits;
        }
    }
}
