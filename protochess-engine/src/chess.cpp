#include <iostream>

#include "shared/chess.h"
#include "bitsetUtil.h"
#include "movegen.h"
#include "moverules.h"
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>

Chess::Chess(int width, int height) : dimensions({width, height}), board(width, height) {}

Chess::Chess() : dimensions({8, 8}), board(8, 8) {
}

std::string Chess::toString() {
    std::string returnString;
    for (auto &pair : players) {
        returnString += pair.second.getName();
        returnString += "\n";
    }

    for (int y = dimensions.height - 1; y >= 0; y--) {
        for (int x = 0; x < dimensions.width; x++) {
            bool pieceHere = false;
            for (auto &pair : players) {
                std::map<boost::uuids::uuid, Piece> pieces = pair.second.getPieces();
                for (auto &playerPair : pieces) {
                    if (playerPair.second.getBitset()[bitsetUtil::getIndex(dimensions.width, {x, y})]) {
                        returnString += playerPair.second.getCharRep();
                        returnString += " ";
                        pieceHere = true;
                        break;
                    }
                }
                if (pieceHere) break;
            }
            if (!pieceHere) {
                returnString += ". ";
            }
        }
        returnString += '\n';
    }
    return returnString;
}

int Chess::registerPlayer(std::string playerName) {
    players.insert({playerCounter, Player(playerName)});
    playerCounter++;
    return playerCounter - 1;
}


void Chess::buildClassicSet() {
    reset();
    board = Board(8, 8);
    dimensions = {8, 8};
    registerPlayer("White"); //player num 0
    registerPlayer("Black"); //player num 1

    char wPieces[] = {
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', 'Q', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '
    };

    char bPieces[] = {
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
            'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'
    };

    players.at(0).setPieces(charToPieceBitsets(wPieces));
    players.at(0).setMovementMap(charToKnownMP(wPieces));
    players.at(1).setPieces(charToPieceBitsets(bPieces));
    players.at(1).setMovementMap(charToKnownMP(bPieces));

    board.updateAllPieces(players);

    movegen::generateMoves(players.at(0), board);
    movegen::generateMoves(players.at(1), board);
}

std::map<boost::uuids::uuid, Piece> Chess::charToPieceBitsets(const char *pieces) {
    std::map<boost::uuids::uuid, Piece> returnMap;
    boost::uuids::random_generator generator;

    int index = 0;
    for (int y = dimensions.height - 1; y >= 0; y--) {
        for (int x = 0; x < dimensions.width; x++) {
            if (pieces[index] != ' ') {
                char charHere = pieces[index];
                boost::uuids::uuid id = generator();
                returnMap.insert(
                        std::make_pair(id,
                                       Piece(id,
                                             boost::dynamic_bitset<>(dimensions.width * dimensions.height),
                                             charHere,
                                             {x, y},
                                             bitsetUtil::getIndex(dimensions.width, {x, y})))
                );

                int i = bitsetUtil::getIndex(dimensions.width, {x, y});
                returnMap.at(id).setBitset(returnMap.at(id).getBitset().set(i, true));
            }
            index++;
        }
    }

    return returnMap;
}

void Chess::reset() {
    board = Board(0, 0);
    players.clear();
    playerCounter = 0;
    whosTurn = 0;
}

std::map<char, MovementPattern> Chess::charToKnownMP(const char *pieces) {
    std::map<char, MovementPattern> returnMap;
    int index = 0;
    for (int y = dimensions.height - 1; y >= 0; y--) {
        for (int x = 0; x < dimensions.width; x++) {
            if (pieces[index] != ' ') {
                char charHere = pieces[index];
                //We know how to handle this type
                if (moverules::rules.count(charHere) != 0) {
                    returnMap.insert(
                            std::make_pair(charHere, moverules::rules.at(charHere))
                    );
                }
            }
            index++;
        }
    }

    return returnMap;
}


