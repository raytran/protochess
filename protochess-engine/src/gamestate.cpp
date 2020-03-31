//
// Created by raytran on 3/26/20.
//

#include <iostream>
#include <utility>
#include <chrono>
#include "gamestate.h"
#include "bitsetutil.h"
#include "movegen.h"
#include <boost/lexical_cast.hpp>
#include <boost/uuid/uuid_io.hpp>

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
        std::shared_ptr<Piece> piece = move.sourcePiece;
        if (piece != nullptr) {
            //Move the piece
            piece->setMovedBefore(piece->getLastMovedBefore());
            piece->setLocation(move.locationDelta.start,
                               bitsetUtil::getIndex(board.getWidth(), move.locationDelta.start));

            //Promotions
            if (move.promotion) {
                piece->setCharRep(piece->getLastCharRep());
            }
            if (move.capture) {
                std::shared_ptr<Piece> captured = move.targetPiece;
                players.at(captured->getOwner()).addPiece(captured);
            } else if (move.castleKingSide || move.castleQueenSide) {
                //Move the rook
                std::shared_ptr<Piece> rook = move.targetPiece;
                Location rookLoc = move.locationDelta.end;
                if (move.castleKingSide) {
                    rookLoc.x += 1;
                } else {
                    rookLoc.x -= 2;
                }
                rook->setLocation(rookLoc, bitsetUtil::getIndex(board.getWidth(), rookLoc));
                rook->setMovedBefore(rook->getLastMovedBefore());
                //Undo disable this player's castling rights
                players.at(piece->getOwner()).enableCastleRights();
            }
        } else {
            std::cerr << "\nUNMAKE NULLPTR\n";
            std::cerr << move.locationDelta.start.x;
            std::cerr << move.locationDelta.start.y;
            std::cerr << move.locationDelta.end.x;
            std::cerr << move.locationDelta.end.y;
            std::cerr << "---\n";
        }
        update();
    }

    void GameState::makeMove(const Move &move) {

        std::shared_ptr<Piece> piece = move.sourcePiece;
        //Move the piece
        if (piece != nullptr) {

            //Promotions
            if (move.promotion) {
                piece->setLastCharRep(piece->getCharRep());
                piece->setCharRep(move.promotedType);
            }

            //Captures and castles cannot both happen at the same time
            if (move.capture) {

                std::shared_ptr<Piece> captured = move.targetPiece;
                players.at(captured->getOwner()).removePiece(captured->getId());
            } else if (move.castleKingSide || move.castleQueenSide) {
                //Move the rook
                std::shared_ptr<Piece> rook = move.targetPiece;
                Location rookLoc = move.locationDelta.end;
                if (move.castleKingSide) {
                    rookLoc.x -= 1;
                } else {
                    rookLoc.x += 1;
                }
                rook->setLastMovedBefore(rook->getLastMovedBefore());
                rook->setLocation(rookLoc, bitsetUtil::getIndex(board.getWidth(), rookLoc));
                rook->setMovedBefore(true);
                //Disable this player's castling rights
                players.at(piece->getOwner()).disableCastleRights();
            }
            //Move the piece
            piece->setLastMovedBefore(piece->getMovedBefore());
            piece->setLocation(move.locationDelta.end, bitsetUtil::getIndex(board.getWidth(), move.locationDelta.end));
            piece->setMovedBefore(true);
        } else {

            std::cerr << "\n";
            std::cerr << move.locationDelta.start.x;
            std::cerr << move.locationDelta.start.y;
            std::cerr << move.locationDelta.end.x;
            std::cerr << move.locationDelta.end.y;
            std::cerr << "\n";
            std::cerr << "WE SHOULD NEVER BE HERE";
        }
        update();
    }

    std::shared_ptr<Piece> GameState::pieceAt(Location loc) {
        for (auto &x:players) {
            std::shared_ptr<Piece> piece = x.second.getPieceAt(loc);
            if (piece != nullptr) {
                return piece;
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
        players.insert({playerCounter, Player(playerCounter, std::move(name))});
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
                if (movegen::isMoveValid(y, *this, playerNum, board)) {
                    //This move is ok; does not brings player into check
                    if (moves.count(x.first) == 0) {
                        moves.insert({x.first, std::unordered_set<Move>()});
                    }
                    moves.at(x.first).insert(y);
                }
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

    std::unordered_set<int> GameState::getCheckMatedPlayers() {
        std::unordered_set<int> returnSet = {};
        for (auto &x : movegen::playersInCheck(*this, board)) {
            if (generateMoves(x).empty()) {
                returnSet.insert(x);
            }
        }
        return returnSet;
    }

    std::unordered_set<int> GameState::getCheckedPlayers() {
        return movegen::playersInCheck(*this, board);
    }

    Board GameState::getBoard() {
        return board;
    }

    namespace {
        unsigned long long perft_(int depth, int whosTurn, GameState &gameState) {
            if (depth == 0) return 1;
            unsigned long long nodes = 0;

            std::map<boost::uuids::uuid, std::unordered_set<Move>>
                    pseudoMoves = gameState.generateMoves(whosTurn);

            for (auto &x:pseudoMoves) {
                for (auto &y:x.second) {
                    gameState.makeMove(y);
                    nodes += perft_(depth - 1, (int) ((whosTurn + 1) % gameState.getPlayerMap().size()), gameState);
                    gameState.unmakeMove(y);
                }
            }

            return nodes;
        }
    }

    unsigned long long GameState::perft(int depth) {
        return perft_(depth, 0, *this);
    }
}



