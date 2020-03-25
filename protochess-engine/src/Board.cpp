//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "Board.h"
#include "bitsetUtil.h"

Board::Board(int width, int height) : dimensions({width, height}),
                                      allPieces(width * height),
                                      leftMostFile(width * height),
                                      rightMostFile(width * height) {
    //Initialize left&rightmost file masks
    for (int i = 0; i < height; i++) {
        leftMostFile.set(bitsetUtil::getIndex(width, {0, i}), true);
        rightMostFile.set(bitsetUtil::getIndex(width, {width - 1, i}), true);
    }


    //Initialize ray attacks
    rayAttacks.insert({NORTH, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({EAST, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({SOUTH, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({WEST, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({NORTHEAST, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({NORTHWEST, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({SOUTHEAST, std::vector<boost::dynamic_bitset<>>()});
    rayAttacks.insert({SOUTHWEST, std::vector<boost::dynamic_bitset<>>()});


    int numBits = width * height;
    rayAttacks.at(NORTH).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(EAST).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(SOUTH).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(WEST).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(NORTHEAST).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(NORTHWEST).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(SOUTHEAST).assign(numBits, boost::dynamic_bitset<>(numBits));
    rayAttacks.at(SOUTHWEST).assign(numBits, boost::dynamic_bitset<>(numBits));

    std::vector<boost::dynamic_bitset<>> &north = rayAttacks.at(NORTH);
    std::vector<boost::dynamic_bitset<>> &east = rayAttacks.at(EAST);
    std::vector<boost::dynamic_bitset<>> &south = rayAttacks.at(SOUTH);
    std::vector<boost::dynamic_bitset<>> &west = rayAttacks.at(WEST);

    std::vector<boost::dynamic_bitset<>> &northEast = rayAttacks.at(NORTHEAST);
    std::vector<boost::dynamic_bitset<>> &northWest = rayAttacks.at(NORTHWEST);
    std::vector<boost::dynamic_bitset<>> &southEast = rayAttacks.at(SOUTHEAST);
    std::vector<boost::dynamic_bitset<>> &southWest = rayAttacks.at(SOUTHWEST);
    //Generate lookup tables
    //lookup tables do not contain same square
    for (int x = 0; x < width; x++) {
        for (int y = 0; y < height; y++) {
            int index = bitsetUtil::getIndex(width, {x, y});

            //NORTH LOOKUP TABLE
            for (int j = y + 1; j < height; j++) {
                int newIndex = bitsetUtil::getIndex(width, {x, j});
                north.at(index).set(newIndex, true);
            }


            //SOUTH LOOKUP TABLE
            for (int j = y - 1; j >= 0; j--) {
                int newIndex = bitsetUtil::getIndex(width, {x, j});
                south.at(index).set(newIndex, true);
            }



            //EAST LOOKUP TABLE
            for (int j = x + 1; j < width; j++) {
                int newIndex = bitsetUtil::getIndex(width, {j, y});
                east.at(index).set(newIndex, true);
            }


            //WEST LOOKUP TABLE
            for (int j = x - 1; j >= 0; j--) {
                int newIndex = bitsetUtil::getIndex(width, {j, y});
                west.at(index).set(newIndex, true);
            }

            //NORTHEAST LOOKUP TABLE
            int x2 = x + 1;
            int y2 = y + 1;
            while (x2 < width && y2 < height) {
                int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                northEast.at(index).set(newIndex, true);
                x2++;
                y2++;
            }


            //NORTHWEST LOOKUP TABLE
            x2 = x - 1;
            y2 = y + 1;
            while (x2 >= 0 && y2 < height) {
                int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                northWest.at(index).set(newIndex, true);
                x2--;
                y2++;
            }


            //SOUTHEAST LOOKUP TABLE
            x2 = x + 1;
            y2 = y - 1;
            while (x2 < width && y2 >= 0) {
                int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                southEast.at(index).set(newIndex, true);
                x2++;
                y2--;
            }


            //SOUTHWEST LOOKUP TABLE
            x2 = x - 1;
            y2 = y - 1;
            while (x2 >= 0 && y2 >= 0) {
                int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                southWest.at(index).set(newIndex, true);
                x2--;
                y2--;
            }
        }
    }
}

boost::dynamic_bitset<> Board::getRightMostFile() const {
    return rightMostFile;
}

boost::dynamic_bitset<> Board::getLeftMostFile() const {
    return leftMostFile;
}

int Board::getWidth() const {
    return dimensions.width;
}

int Board::getHeight() const {
    return dimensions.height;
}

void Board::updateAllPieces(const std::map<int, Player> &players) {
    allPieces.reset();
    for (auto const &x : players) {
        std::map<char, boost::dynamic_bitset<>> pPieces = x.second.getPieces();
        for (auto const &x:pPieces) {
            allPieces |= x.second;
        }
    }
}

boost::dynamic_bitset<> Board::getAllPieces() const {
    return allPieces;
}

Dimensions Board::getDimensions() const {
    return dimensions;
}




