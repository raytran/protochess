//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "board.h"
#include "bitsetutil.h"

namespace protochess_engine {
    using boost::dynamic_bitset;
    Board::Board(int width, int height) : dimensions({width, height}),
                                          allPieces(width * height),
                                          leftMostFile(width * height),
                                          rightMostFile(width * height) {
        //Initialize Tiles
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                tiles.push_back({{x, y}, (x + y) % 2 == 0 ? 'b' : 'w'});
            }
        }




        //Initialize left&rightmost file masks
        for (int i = 0; i < height; i++) {
            leftMostFile.set(bitsetUtil::getIndex(width, {0, i}), true);
            rightMostFile.set(bitsetUtil::getIndex(width, {width - 1, i}), true);
        }

        //Initialize left & right file masks
        dynamic_bitset<> cumulativeLeft(width * height);
        dynamic_bitset<> cumulativeRight(width * height);
        for (int i = 0; i < width; i++) {
            dynamic_bitset<> newLeft(width * height);
            dynamic_bitset<> newRight(width * height);
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

        //Initialize ray attacks
        rayAttacks.insert({NORTH, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({EAST, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({SOUTH, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({WEST, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({NORTHEAST, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({NORTHWEST, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({SOUTHEAST, std::vector<dynamic_bitset<>>()});
        rayAttacks.insert({SOUTHWEST, std::vector<dynamic_bitset<>>()});

        int numBits = width * height;
        rayAttacks.at(NORTH).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(EAST).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(SOUTH).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(WEST).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(NORTHEAST).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(NORTHWEST).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(SOUTHEAST).assign(numBits, dynamic_bitset<>(numBits));
        rayAttacks.at(SOUTHWEST).assign(numBits, dynamic_bitset<>(numBits));

        //Generate lookup tables
        //lookup tables do not contain same square
        for (int x = 0; x < width; x++) {
            for (int y = 0; y < height; y++) {
                int index = bitsetUtil::getIndex(width, {x, y});

                //NORTH LOOKUP TABLE
                for (int j = y + 1; j < height; j++) {
                    int newIndex = bitsetUtil::getIndex(width, {x, j});
                    rayAttacks.at(NORTH)[index].set(newIndex, true);
                }
                //SOUTH LOOKUP TABLE
                for (int j = y - 1; j >= 0; j--) {
                    int newIndex = bitsetUtil::getIndex(width, {x, j});
                    rayAttacks.at(SOUTH)[index].set(newIndex, true);
                }
                //EAST LOOKUP TABLE
                for (int j = x + 1; j < width; j++) {
                    int newIndex = bitsetUtil::getIndex(width, {j, y});
                    rayAttacks.at(EAST)[index].set(newIndex, true);
                }
                //WEST LOOKUP TABLE
                for (int j = x - 1; j >= 0; j--) {
                    int newIndex = bitsetUtil::getIndex(width, {j, y});
                    rayAttacks.at(WEST)[index].set(newIndex, true);
                }
                //NORTHEAST LOOKUP TABLE
                int x2 = x + 1;
                int y2 = y + 1;
                while (x2 < width && y2 < height) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    rayAttacks.at(NORTHEAST)[index].set(newIndex, true);
                    x2++;
                    y2++;
                }
                //NORTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y + 1;
                while (x2 >= 0 && y2 < height) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    rayAttacks.at(NORTHWEST)[index].set(newIndex, true);
                    x2--;
                    y2++;
                }
                //SOUTHEAST LOOKUP TABLE
                x2 = x + 1;
                y2 = y - 1;
                while (x2 < width && y2 >= 0) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    rayAttacks.at(SOUTHEAST)[index].set(newIndex, true);
                    x2++;
                    y2--;
                }
                //SOUTHWEST LOOKUP TABLE
                x2 = x - 1;
                y2 = y - 1;
                while (x2 >= 0 && y2 >= 0) {
                    int newIndex = bitsetUtil::getIndex(width, {x2, y2});
                    rayAttacks.at(SOUTHWEST)[index].set(newIndex, true);
                    x2--;
                    y2--;
                }
            }
        }
    }

    dynamic_bitset<> Board::getRightMostFile() const {
        return rightMostFile;
    }

    dynamic_bitset<> Board::getLeftMostFile() const {
        return leftMostFile;
    }

    int Board::getWidth() const {
        return dimensions.width;
    }

    int Board::getHeight() const {
        return dimensions.height;
    }

    void Board::update(std::map<int, Player> &players) {
        allPieces.reset();
        for (auto &x : players) {
            std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pPieces = x.second.getPieces();
            for (auto &x:pPieces) {
                allPieces |= x.second->getBitset();
            }
        }
    }

    const dynamic_bitset<> &Board::getAllPieces() const {
        return allPieces;
    }

    Dimensions Board::getDimensions() const {
        return dimensions;
    }

    dynamic_bitset<> Board::getRayAttack(const Direction &dir, const int &index) const {
        return rayAttacks.at(dir)[index];
    }

    dynamic_bitset<> Board::getRightMask(int numCols) const {
        if (numCols == 0) {
            return dynamic_bitset<>(getWidth() * getHeight());
        }
        return rightMasks[numCols - 1];
    }

    dynamic_bitset<> Board::getLeftMask(int numCols) const {
        if (numCols == 0) {
            return dynamic_bitset<>(getWidth() * getHeight());
        }
        return leftMasks[numCols - 1];
    }

    std::vector<Tile> &Board::getTiles() {
        return tiles;
    }
}
