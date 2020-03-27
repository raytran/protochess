//
// Created by raytran on 3/26/20.
//

#include <iostream>
#include <utility>
#include "gamestate.h"
#include "bitsetutil.h"
#include "movegen.h"

namespace protochess_engine {
    std::string GameState::toString() {
        std::string returnString;
        returnString += "Players: \n";
        for (auto &pair : players) {
            returnString += pair.second.getName();
            returnString += "\n";
        }
        returnString += "-------- \n";
        for (int y = dimensions.height - 1; y >= 0; y--) {
            for (int x = 0; x < dimensions.width; x++) {
                bool pieceHere = false;
                for (auto &pair : players) {
                    std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieces = pair.second.getPieces();
                    for (auto &playerPair : pieces) {
                        if (playerPair.second->getBitset()[bitsetUtil::getIndex(dimensions.width, {x, y})]) {
                            returnString += playerPair.second->getCharRep();
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

    void GameState::unmakeMove(const Move &move) {
        if (move.capture && move.capturedPiece != nullptr) {
            std::shared_ptr<Piece> captured = move.capturedPiece;
            players.at(captured->getOwner()).addPiece(captured);
        }
        std::shared_ptr<Piece> piece = pieceAt(move.locationDelta.end);
        //Move the piece
        if (piece != nullptr) {
            piece->setLocation(move.locationDelta.start,
                               bitsetUtil::getIndex(board.getWidth(), move.locationDelta.start));
        }
        update();
    }

    void GameState::makeMove(const Move &move) {
        if (move.capture && move.capturedPiece != nullptr) {
            std::shared_ptr<Piece> captured = move.capturedPiece;
            players.at(captured->getOwner()).removePiece(captured->getId());
        }
        std::shared_ptr<Piece> piece = pieceAt(move.locationDelta.start);
        //Move the piece
        if (piece != nullptr) {
            piece->setLocation(move.locationDelta.end, bitsetUtil::getIndex(board.getWidth(), move.locationDelta.end));
        }

        update();
    }

    std::shared_ptr<Piece> GameState::pieceAt(Location loc) {
        for (auto &x:players) {
            boost::uuids::uuid idHere = x.second.getPieceIdAt(loc);
            if (!idHere.is_nil()) {
                return x.second.getPieces().at(idHere);
            }
        }
        return nullptr;
    }

    void GameState::update() {
        board.update(players);
        for (auto &x:players) {
            x.second.update();
        }
    }

    int GameState::registerPlayer(std::string name) {
        players.insert({playerCounter, Player(name)});
        playerCounter++;
        return playerCounter - 1;
    }

    std::map<boost::uuids::uuid, std::unordered_set<Move>> GameState::generateMoves(int playerNum) {
        std::map<boost::uuids::uuid, std::unordered_set<Move>> moves = {};
        std::map<boost::uuids::uuid, std::unordered_set<Move>> pseudoMoves = movegen::generatePseudoLegalMoves(
                *this,
                players.at(playerNum),
                board);

        for (auto &x:pseudoMoves) {
            for (const auto &y:x.second) {
                makeMove(y);
                std::unordered_set<int> playersInCheck = movegen::playersInCheck(*this, board);
                if (playersInCheck.find(playerNum) == playersInCheck.end()) {
                    //This move is ok; does not brings player into check
                    if (moves.count(x.first) == 0) {
                        moves.insert({x.first, std::unordered_set<Move>()});
                    }
                    moves.at(x.first).insert(y);
                }
                unmakeMove(y);
            }
        }
        return moves;
    }

    int GameState::getHeight() {
        return dimensions.height;
    }

    int GameState::getWidth() {
        return dimensions.width;
    }

    const Dimensions &GameState::getDimensions() {
        return dimensions;
    }

    void GameState::incrementTurn() {
        whosTurn = (int) ((whosTurn + 1) % players.size());
    }

    int GameState::getWhosTurn() {
        return whosTurn;
    }

    void GameState::setPieces(int playerNum, std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap) {
        players.at(playerNum).setPieces(std::move(pieceMap));
        update();
    }

    void GameState::setMovementMap(int playerNum, std::map<char, MovementPattern> moveMap) {
        players.at(playerNum).setMovementMap(std::move(moveMap));
        update();
    }

    void GameState::setCaptureMap(int playerNum, std::map<char, MovementPattern> captureMap) {
        players.at(playerNum).setCaptureMap(std::move(captureMap));
        update();
    }

    GameState::GameState(Dimensions dim) : board(dim.width, dim.height), dimensions(dim) {
    }

    std::map<int, Player> &GameState::getPlayerMap() {
        return players;
    }
}
