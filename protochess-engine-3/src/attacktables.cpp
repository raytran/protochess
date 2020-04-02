//
// Created by raytran on 3/31/20.
//
#include "attacktables.h"
#include "bitboardutil.h"

using boost::multiprecision::bit_set;
namespace protochess_engine{
    AttackTables::AttackTables(Dimensions dimensions):dimensions(dimensions) {
        int width = dimensions.width;
        int height = dimensions.height;
        numBits = width * height;
        //Initialize left & right file masks
        bitboard cumulativeLeft = 0;
        bitboard cumulativeRight = 0;
        for (int i = 0; i < width; i++) {
            bitboard newLeft = 0;
            bitboard newRight = 0;
            newLeft |= cumulativeLeft;
            newRight |= cumulativeRight;

            for (int j = 0; j < height; j++) {
                bit_set(newLeft,bitboardutil::getIndex(width,i,j));
                bit_set(newRight,bitboardutil::getIndex(width,width - i - 1,j));
            }
            rightMasks.push_back(newRight);
            leftMasks.push_back(newLeft);
            cumulativeLeft |= newLeft;
            cumulativeRight |= newRight;
        }
        //Initialize attacks
        northAttacks.assign(numBits, 0);
        eastAttacks.assign(numBits, 0);
        southAttacks.assign(numBits, 0);
        westAttacks.assign(numBits, 0);
        northEastAttacks.assign(numBits, 0);
        northWestAttacks.assign(numBits, 0);
        southEastAttacks.assign(numBits, 0);
        southWestAttacks.assign(numBits, 0);

        //Generate lookup tables
        //lookup tables do not contain same square
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                int index = bitboardutil::getIndex(width, x, y);

                bit_set(boundaryMask,index);

                //NORTH LOOKUP TABLE
                for (int j = y + 1; j < height; j++) {
                    int newIndex = bitboardutil::getIndex(width, x, j);
                    bit_set(northAttacks[index],newIndex);
                }
                //SOUTH LOOKUP TABLE
                for (int j = y - 1; j >= 0; j--) {
                    int newIndex = bitboardutil::getIndex(width, x, j);
                    bit_set(southAttacks[index],newIndex);
                }
                //EAST LOOKUP TABLE
                for (int j = x + 1; j < width; j++) {
                    int newIndex = bitboardutil::getIndex(width, j, y);
                    bit_set(eastAttacks[index],newIndex);
                }
                //WEST LOOKUP TABLE
                for (int j = x - 1; j >= 0; j--) {
                    int newIndex = bitboardutil::getIndex(width, j, y);
                    bit_set(westAttacks[index],newIndex);
                }
                //NORTHEAST LOOKUP TABLE
                int x2 = x + 1;
                int y2 = y + 1;
                while (x2 < width && y2 < height) {
                    int newIndex = bitboardutil::getIndex(width, x2, y2);
                    bit_set(northEastAttacks[index],newIndex);
                    x2++;
                    y2++;
                }
                //NORTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y + 1;
                while (x2 >= 0 && y2 < height) {
                    int newIndex = bitboardutil::getIndex(width, x2, y2);
                    bit_set(northWestAttacks[index],newIndex);
                    x2--;
                    y2++;
                }
                //SOUTHEAST LOOKUP TABLE
                x2 = x + 1;
                y2 = y - 1;
                while (x2 < width && y2 >= 0) {
                    int newIndex = bitboardutil::getIndex(width, x2, y2);
                    bit_set(southEastAttacks[index],newIndex);
                    x2++;
                    y2--;
                }
                //SOUTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y - 1;
                while (x2 >= 0 && y2 >= 0) {
                    int newIndex = bitboardutil::getIndex(width, x2, y2);
                    bit_set(southWestAttacks[index],newIndex);
                    x2--;
                    y2--;
                }
            }
        }
    }
    const bitboard &AttackTables::getRayAttack(const Direction &dir, const int &index) const {
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

    const bitboard &AttackTables::getRightMask(int numCols) const {
        if (numCols == 0) {
            return zero;
        }
        return rightMasks[numCols - 1];
    }

    const bitboard &AttackTables::getLeftMask(int numCols) const {
        if (numCols == 0) {
            return zero;
        }
        return leftMasks[numCols - 1];
    }

    int AttackTables::getWidth() const{
        return dimensions.width;
    }

    int AttackTables::getHeight() const{
        return dimensions.height;
    }

    int AttackTables::getNumBits() const{
        return numBits;
    }


    const bitboard &AttackTables::getBoundaryMask() const{
        return boundaryMask;
    }
}

