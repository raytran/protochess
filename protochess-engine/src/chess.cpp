#include <iostream>

#include "shared/chess.h"
#include "bitsetUtil.h"
#include "movegen.h"

Chess::Chess(int width, int height) : dimensions({width, height}), board(width, height) {}

Chess::Chess() : dimensions({8,8}), board(8, 8) {
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
                std::map<char, boost::dynamic_bitset<>> playerPiece = pair.second.getPieces();
                for (auto &playerPair : playerPiece) {
                    if (playerPair.second[bitsetUtil::getIndex(dimensions.width, {x, y})]) {
                        returnString += playerPair.first;
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
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P',
            'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'
    };

    char bPieces[] = {
            'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r',
            'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '
    };

    players.at(0).setPieces(charToPieceBitsets(wPieces));
    players.at(1).setPieces(charToPieceBitsets(bPieces));
    board.updateAllPieces(players);

    movegen::generateMoves(players.at(0), board);
}

std::map<char, boost::dynamic_bitset<>> Chess::charToPieceBitsets(const char *pieces) {
    std::map<char, boost::dynamic_bitset<>> returnMap;
    int index = 0;
    for (int y = dimensions.height - 1; y >= 0; y--) {
        for (int x = 0; x < dimensions.width; x++) {
            if (pieces[index] != ' ') {

                char charHere = pieces[index];
                if (returnMap.count(charHere) == 0) {
                    returnMap.insert(
                            std::make_pair(charHere, boost::dynamic_bitset<>(dimensions.width * dimensions.height)));
                }

                int i = bitsetUtil::getIndex(dimensions.width, {x, y});
                returnMap.at(charHere).set(i, true);
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


