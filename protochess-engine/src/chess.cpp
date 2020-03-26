#include <iostream>

#include "shared/chess.h"
#include "bitsetUtil.h"
#include "movegen.h"
#include "piecerules.h"
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

    players.at(0).setPieces(charToPieces(0, wPieces));
    players.at(0).setMovementMap(charToKnownMP(piecerules::moveRules, wPieces));
    players.at(0).setCaptureMap(charToKnownMP(piecerules::captureRules, wPieces));

    players.at(1).setPieces(charToPieces(1, bPieces));
    players.at(1).setMovementMap(charToKnownMP(piecerules::moveRules, bPieces));
    players.at(1).setCaptureMap(charToKnownMP(piecerules::captureRules, bPieces));

    board.update(players);

}

std::map<boost::uuids::uuid, Piece> Chess::charToPieces(int owner, const char *pieces) {
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
                                       Piece(owner,
                                             id,
                                             boost::dynamic_bitset<>(dimensions.width * dimensions.height),
                                             charHere,
                                             {x, y},
                                             bitsetUtil::getIndex(dimensions.width, {x, y})))
                );

                int i = bitsetUtil::getIndex(dimensions.width, {x, y});
                returnMap.at(id).setLocation({x, y}, i);
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

std::map<char, MovementPattern> Chess::charToKnownMP(std::map<char, MovementPattern> &dictionary, const char *pieces) {
    std::map<char, MovementPattern> returnMap;
    int index = 0;
    for (int y = dimensions.height - 1; y >= 0; y--) {
        for (int x = 0; x < dimensions.width; x++) {
            if (pieces[index] != ' ') {
                char charHere = pieces[index];
                //We know how to handle this type
                if (dictionary.count(charHere) != 0) {
                    returnMap.insert(
                            std::make_pair(charHere, dictionary.at(charHere))
                    );
                }
            }
            index++;
        }
    }
    return returnMap;
}

bool Chess::takeTurn(int startX, int startY, int endX, int endY) {
    Location start = {startX, startY};
    Location end = {endX, endY};
    LocationDelta delta = {start, end};
    boost::uuids::uuid idHere = players.at(whosTurn).getPieceIdAt(start);
    if (!idHere.is_nil()) {
        //There exists a piece at the start index
        std::map<boost::uuids::uuid, std::vector<Move>> moves = movegen::generateMoves(players.at(whosTurn), board);
        //This piece has viable moves; check if startLoc->endLoc is one of them
        if (moves.count(idHere) != 0) {
            for (auto &x : moves.at(idHere)) {
                if (x.locationDelta == delta) {
                    //Viable move!
                    //Perform move
                    makeMove(x);
                    whosTurn = (int) ((whosTurn + 1) % players.size());
                    return true;
                }
            }
        }
    }
    return false;
}

void Chess::makeMove(Move move) {
    if (move.capture) {
        std::cout << "PIECE CAPTURED!";
        Piece &captured = pieceAt(move.locationDelta.end);
        players.at(captured.getOwner()).removePiece(captured.getId());
    }
    Piece &piece = pieceAt(move.locationDelta.start);
    //Move the piece
    piece.setLocation(move.locationDelta.end, bitsetUtil::getIndex(board.getWidth(), move.locationDelta.end));

    update();
}

Piece &Chess::pieceAt(Location loc) {
    for (auto &x:players) {
        boost::uuids::uuid idHere = x.second.getPieceIdAt(loc);
        if (!idHere.is_nil()) {
            return x.second.getPieces().at(idHere);
        }
    }
    throw std::runtime_error("Piece does not exist at loc");
}

void Chess::update() {
    board.update(players);
    for (auto &x:players) {
        x.second.update();
    }
}


