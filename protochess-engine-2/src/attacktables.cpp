//
// Created by raytran on 3/31/20.
//
#include "attacktables.h"
#include "bitsetutil.h"

namespace protochess_engine{
    AttackTables::AttackTables(Dimensions dimensions):dimensions(dimensions) {
        int width = dimensions.width;
        int height = dimensions.height;
        int numBits = width * height;
        //Initialize left & right file masks
        dynamic_bitset<> cumulativeLeft(numBits);
        dynamic_bitset<> cumulativeRight(numBits);
        for (int i = 0; i < width; i++) {
            dynamic_bitset<> newLeft(numBits);
            dynamic_bitset<> newRight(numBits);
            newLeft |= cumulativeLeft;
            newRight |= cumulativeRight;

            for (int j = 0; j < height; j++) {
                newLeft.set(bitsetUtil::getIndex(width, {i, j}), true);
                newRight.set(bitsetUtil::getIndex(width, {width - i - 1, j}), true);
            }
            rightMasks.push_back(newRight);
            leftMasks.push_back(newLeft);
            cumulativeLeft |= newLeft;
            cumulativeRight |= newRight;
        }
        //Initialize attacks
        northAttacks.assign(numBits, dynamic_bitset<>(numBits));
        eastAttacks.assign(numBits, dynamic_bitset<>(numBits));
        southAttacks.assign(numBits, dynamic_bitset<>(numBits));
        westAttacks.assign(numBits, dynamic_bitset<>(numBits));
        northEastAttacks.assign(numBits, dynamic_bitset<>(numBits));
        northWestAttacks.assign(numBits, dynamic_bitset<>(numBits));
        southEastAttacks.assign(numBits, dynamic_bitset<>(numBits));
        southWestAttacks.assign(numBits, dynamic_bitset<>(numBits));

        //Generate lookup tables
        //lookup tables do not contain same square
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                int index = bitsetUtil::getIndex(width, {x, y});

                //NORTH LOOKUP TABLE
                for (int j = y + 1; j < height; j++) {
                    int newIndex = bitsetUtil::getIndex(width, {x, j});
                    northAttacks[index].set(newIndex, true);
                }
                //SOUTH LOOKUP TABLE
                for (int j = y - 1; j >= 0; j--) {
                    int newIndex = bitsetUtil::getIndex(width, {x, j});
                    southAttacks[index].set(newIndex, true);
                }
                //EAST LOOKUP TABLE
                for (int j = x + 1; j < width; j++) {
                    int newIndex = bitsetUtil::getIndex(width, {j, y});
                    eastAttacks[index].set(newIndex, true);
                }
                //WEST LOOKUP TABLE
                for (int j = x - 1; j >= 0; j--) {
                    int newIndex = bitsetUtil::getIndex(width, {j, y});
                    westAttacks[index].set(newIndex, true);
                }
                //NORTHEAST LOOKUP TABLE
                int x2 = x + 1;
                int y2 = y + 1;
                while (x2 < width && y2 < height) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    northEastAttacks[index].set(newIndex, true);
                    x2++;
                    y2++;
                }
                //NORTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y + 1;
                while (x2 >= 0 && y2 < height) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    northWestAttacks[index].set(newIndex, true);
                    x2--;
                    y2++;
                }
                //SOUTHEAST LOOKUP TABLE
                x2 = x + 1;
                y2 = y - 1;
                while (x2 < width && y2 >= 0) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    southEastAttacks[index].set(newIndex, true);
                    x2++;
                    y2--;
                }
                //SOUTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y - 1;
                while (x2 >= 0 && y2 >= 0) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    southWestAttacks[index].set(newIndex, true);
                    x2--;
                    y2--;
                }
            }
        }
    }
    const dynamic_bitset<> &AttackTables::getRayAttack(const Direction &dir, const int &index) const {
        switch(dir){
            case NORTH:
                return northAttacks[index];
            case EAST:
                return eastAttacks[index];
            case SOUTH:
                return southAttacks[index];
            case WEST:
                return westAttacks[index];
            case NORTHEAST:
                return northEastAttacks[index];
            case SOUTHEAST:
                return southEastAttacks[index];
            case NORTHWEST:
                return northWestAttacks[index];
            case SOUTHWEST:
                return southWestAttacks[index];
        }
    }

    dynamic_bitset<> AttackTables::getRightMask(int numCols) const {
        if (numCols == 0) {
            return dynamic_bitset<>(dimensions.width * dimensions.height);
        }
        return rightMasks[numCols - 1];
    }

    dynamic_bitset<> AttackTables::getLeftMask(int numCols) const {
        if (numCols == 0) {
            return dynamic_bitset<>(dimensions.width * dimensions.height);
        }
        return leftMasks[numCols - 1];
    }

    int AttackTables::getWidth() const{
        return dimensions.width;
    }

    int AttackTables::getHeight() const{
        return dimensions.height;
    }
}
