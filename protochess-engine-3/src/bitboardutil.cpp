//
// Created by raytran on 4/1/20.
//

#include "bitboardutil.h"
#include "types.h"

namespace protochess_engine::bitboardutil {
    std::string bitsetToString(bitboard bitboard, const Dimensions &dimensions) {
        std::string returnString;
        int width = dimensions.width;
        int height = dimensions.height;
        for (int y = height - 1; y >= 0; y--) {
            for (int x = 0; x < width; x++) {
                if (bit_test(bitboard, getIndex(width, x, y))) {
                    returnString += "1 ";
                } else {
                    returnString += ". ";
                }
            }
            returnString += "\n";
        }
        return returnString;
    }

    int getIndex(int width, int x, int y) {
        return y * width + x;
    }

    bitboard east(int amt, const bitboard &in, const AttackTables *tables) {
        return (in << amt) & ~tables->getLeftMask(amt);
    }

    bitboard west(int amt, const bitboard &in, const AttackTables *tables) {
        return (in >> amt) & ~tables->getRightMask(amt);
    }

    //Mask with top boundary to prevent placing unnecessary data outside of range
    bitboard north(int amt, const bitboard &in, const AttackTables *tables) {
        return in << (amt * tables->getWidth());
    }

    //Technically masking with the boundary is unnecessary, but doing so anyway
    //In case there is garbage data at index > boundary
    bitboard south(int amt, const bitboard &in, const AttackTables *tables) {
        return in >> (amt * tables->getWidth());
    }

    bitboard translate(const Location &delta, const bitboard &in, const AttackTables *attackTables) {
        bitboard returnBits = in;
        if (delta.x > 0) {
            returnBits = east(delta.x, returnBits, attackTables);
        } else {
            returnBits = west(-delta.x, returnBits, attackTables);
        }
        if (delta.y > 0) {
            returnBits = north(delta.y, returnBits, attackTables);
        } else {
            returnBits = south(-delta.y, returnBits, attackTables);
        }
        return returnBits & attackTables->getBoundaryMask();
    }

    Location getLoc(int width, int index) {
        return {index % width, index / width};
    }

}
