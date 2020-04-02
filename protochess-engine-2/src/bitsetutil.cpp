//
// Created by raytran on 3/31/20.
//

#include "bitsetutil.h"
#include "attacktables.h"

using boost::dynamic_bitset;
namespace protochess_engine::bitsetUtil{
    dynamic_bitset<>
    east(int amt, const dynamic_bitset<> &in, const AttackTables &attackTables) {
        return (in << amt) & ~attackTables.getLeftMask(amt);
    }

    dynamic_bitset<>
    west(int amt, const dynamic_bitset<> &in, const AttackTables &attackTables) {
        return (in >> amt) & ~attackTables.getRightMask(amt);
    }

    dynamic_bitset<>
    north(int amt, const dynamic_bitset<> &in, const AttackTables &attackTables) {
        return in << (amt * attackTables.getWidth());
    }

    dynamic_bitset<>
    south(int amt, const dynamic_bitset<> &in, const AttackTables &attackTables) {
        return in >> (amt * attackTables.getWidth());
    }

    std::string bitsetToString(const boost::dynamic_bitset<> &bitset,
                                           const Dimensions &dimensions) {
        std::string returnString;
        int width = dimensions.width;
        int height = dimensions.height;
        for (int y = height - 1; y >= 0; y--) {
            for (int x = 0; x < width; x++) {
                if (bitset[getIndex(width, {x, y})]) {
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


    boost::dynamic_bitset<> toBitset(const Location &loc, const Dimensions &dimensions) {
        boost::dynamic_bitset<> returnBits(dimensions.width * dimensions.height);
        returnBits.set(getIndex(dimensions.width,loc));
        return returnBits;
    }

    //Like find_first, but for last
    long findLast(const boost::dynamic_bitset<> &bitset) {
        long index = -1;
        for (long i = bitset.size() - 1; i >= 0; i--) {
            if (bitset[i]) {
                index = i;
                break;
            }
        }
        return index;
    }
    dynamic_bitset<> translate(Location delta, const dynamic_bitset<> &in, const AttackTables &attackTables) {
        dynamic_bitset<> returnBits(in);
        int dx = delta.x;
        int dy = delta.y;
        if (dx > 0) {
            returnBits = east(dx, returnBits, attackTables);
        } else {
            returnBits = west(-dx, returnBits, attackTables);
        }
        if (dy > 0) {
            returnBits = north(dy, returnBits, attackTables);
        } else {
            returnBits = south(-dy, returnBits, attackTables);
        }
        return returnBits;
    }
}
