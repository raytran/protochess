//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "movegen.h"
#include "bitsetutil.h"
#include "piecerules.h"
#include "types.h"
#include <chrono>

namespace protochess_engine {
    using namespace bitsetUtil;
    namespace movegen {
        namespace {
            boost::dynamic_bitset<>
            calculatePositiveAttacks(const Direction &dir, Board &board, const boost::dynamic_bitset<> &positiveAttack,
                                     const boost::dynamic_bitset<> &allPieces) {
                boost::dynamic_bitset<> returnAttack(positiveAttack);
                boost::dynamic_bitset<> blockers = positiveAttack & allPieces;
                if (blockers.any()) {
                    //Piece is being blocked
                    returnAttack = positiveAttack ^ board.getRayAttack(dir, blockers.find_first());
                }
                return returnAttack;
            }

            boost::dynamic_bitset<>
            calculateNegativeAttacks(const Direction &dir, Board &board, const boost::dynamic_bitset<> &negativeAttack,
                                     const boost::dynamic_bitset<> &allPieces) {
                boost::dynamic_bitset<> returnAttack(negativeAttack);
                boost::dynamic_bitset<> blockers = negativeAttack & allPieces;
                if (blockers.any()) {
                    //Piece is being blocked
                    returnAttack = negativeAttack ^ board.getRayAttack(dir, bitsetUtil::findLast(blockers));
                }
                return returnAttack;
            }

            std::vector<LocationDelta> bitboardsToDeltas(const Dimensions &dimensions,
                                                         const boost::dynamic_bitset<> &onePiece,
                                                         boost::dynamic_bitset<> destinations) {

                std::vector<LocationDelta> returnSet;
                Location start = bitsetUtil::getLoc(dimensions.width, onePiece.find_first());

                while (destinations.find_first() != boost::dynamic_bitset<>::npos) {
                    size_type index = destinations.find_first();
                    Location end = bitsetUtil::getLoc(dimensions.width, index);
                    returnSet.push_back({start, end});
                    destinations.set(index, false);
                }
                return returnSet;
            }

            std::map<boost::uuids::uuid, std::unordered_set<Move>>
            generatePseudoMoveCaptures_(GameState &gameState, bool captures, Player &player, Board &board) {
                std::map<boost::uuids::uuid, std::unordered_set<Move>> returnSet = {};

                std::map<boost::uuids::uuid, std::shared_ptr<Piece>> playerPieceMap = player.getPieces();
                std::map<char, MovementPattern> &playerMPmap = captures ? player.getCaptureMap()
                                                                        : player.getMovementMap();
                boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());

                //Generate all of this player's pieces
                boost::dynamic_bitset<> thisPlayerPieces = player.getAllPiecesBitset();

                //generate enemies
                boost::dynamic_bitset<> enemyPieces;
                enemyPieces = allPlayerPieces & (~thisPlayerPieces);


                for (auto &x:playerPieceMap) {
                    //Determine which movement pattern to use
                    MovementPattern thisMP;
                    if (playerMPmap.count(x.second->getCharRep()) != 0) {
                        thisMP = playerMPmap.at(x.second->getCharRep());
                    } else {
                        std::cerr << "WARNING: unknown piece MP; defaulting to king-style movement\n";
                        thisMP = piecerules::moveRules.at('k');
                    }

                    //Squares that the piece can move to, including captures
                    boost::dynamic_bitset<> validSquares(board.getWidth() * board.getHeight());
                    int locIndex = x.second->getLocationIndex();
                    //SLIDING MOVES:
                    //POSITIVE ATTACKS
                    if (thisMP.north) {
                        validSquares ^= calculatePositiveAttacks(NORTH, board,
                                                                 board.getRayAttack(NORTH, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.east) {
                        validSquares ^= calculatePositiveAttacks(EAST, board,
                                                                 board.getRayAttack(EAST, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.northEast) {
                        validSquares ^= calculatePositiveAttacks(NORTHEAST, board,
                                                                 board.getRayAttack(NORTHEAST, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.northWest) {
                        validSquares ^= calculatePositiveAttacks(NORTHWEST, board,
                                                                 board.getRayAttack(NORTHWEST, locIndex),
                                                                 allPlayerPieces);
                    }

                    //NEGATIVE ATTACKS
                    if (thisMP.south) {
                        validSquares ^= calculateNegativeAttacks(SOUTH, board,
                                                                 board.getRayAttack(SOUTH, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.west) {
                        validSquares ^= calculateNegativeAttacks(WEST, board,
                                                                 board.getRayAttack(WEST, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.southEast) {
                        validSquares ^= calculateNegativeAttacks(SOUTHEAST, board,
                                                                 board.getRayAttack(SOUTHEAST, locIndex),
                                                                 allPlayerPieces);
                    }
                    if (thisMP.southWest) {
                        validSquares ^= calculateNegativeAttacks(SOUTHWEST, board,
                                                                 board.getRayAttack(SOUTHWEST, locIndex),
                                                                 allPlayerPieces);
                    }

                    //NONSLIDING PIECES
                    for (auto &m : thisMP.deltas) {
                        //Edge case pawn movement
                        if ((x.second->getCharRep() == 'p' || x.second->getCharRep() == 'P')
                            && x.second->getMovedBefore()) {
                            if (m.y == 2 || m.y == -2) {
                                continue;
                            }
                        }
                        validSquares ^= bitsetUtil::translate(m, x.second->getBitset(), board);
                    }

                    //Filter out attacks on your own pieces
                    validSquares &= ~thisPlayerPieces;

                    //Convert the valid squares board to moves
                    if (validSquares.any()) {
                        std::vector<LocationDelta> deltas = bitboardsToDeltas(board.getDimensions(),
                                                                              x.second->getBitset(),
                                                                              validSquares);
                        for (auto &delta : deltas) {

                            boost::dynamic_bitset<> singleEndPoint(board.getWidth() * board.getHeight());
                            singleEndPoint.set(bitsetUtil::getIndex(board.getWidth(), delta.end), true);

                            bool captureHere = (singleEndPoint & enemyPieces).any();
                            if (captures == captureHere) {
                                if (returnSet.count(x.first) == 0) {
                                    returnSet.insert({x.first, std::unordered_set<Move>()});
                                }
                                returnSet.at(x.first).insert(
                                        {captures,
                                         gameState.pieceAt(delta.end),
                                         false,
                                         false,
                                         delta}
                                );
                            }
                        }
                    }
                }
                return returnSet;
            }


            std::map<boost::uuids::uuid, std::unordered_set<Move>>
            generateCastlingPseudoMoves_(GameState &gameState,
                                         std::map<boost::uuids::uuid, std::unordered_set<Move>> &pseudoMoves,
                                         Player &player,
                                         Board &board) {
                std::map<boost::uuids::uuid, std::unordered_set<Move>> returnSet = {};

                //Check if this player can castle
                if (player.canCastle()) {
                    boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());
                    //Generate all of this player's pieces
                    boost::dynamic_bitset<> thisPlayerPieces = player.getAllPiecesBitset();
                    //generate enemies
                    boost::dynamic_bitset<> enemyPieces;
                    enemyPieces = allPlayerPieces & (~thisPlayerPieces);


                    //Find the king

                    for (auto &x:player.getPieces()) {
                        //Check if this piece is a king that hasn't moved before
                        //And has ANY valid moves
                        if ((x.second->getCharRep() == 'k' || x.second->getCharRep() == 'K')
                            && pseudoMoves.find(x.first) != pseudoMoves.end()
                            && !x.second->getMovedBefore()) {

                            bool isWhite = x.second->getCharRep() == 'K';
                            //KINGSIDE CASTLING
                            //Check if player can move one square kingside
                            //Find the move to the right, if it exists
                            for (auto &z:pseudoMoves.at(x.first)) {
                                //Make sure this is not a capture move
                                if (!z.capture
                                    && z.locationDelta.end.x - z.locationDelta.start.x == 1
                                    && z.locationDelta.end.y - z.locationDelta.start.y == 0) {
                                    if (isMoveValid(z, gameState, player.getPlayerNum(), board)) {
                                        //Check if the next square is not occupied & if the leftmost square
                                        //Is occupied by a piece that is a rook that hasn't moved yet
                                        Location end = z.locationDelta.end;
                                        end.x += 1;
                                        boost::dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
                                        nextSquare.set(bitsetUtil::getIndex(board.getWidth(), end), true);
                                        Location castlingRookLoc = end;
                                        castlingRookLoc.x += 1;
                                        std::shared_ptr<Piece> castlingRook = gameState.pieceAt(castlingRookLoc);
                                        if (!(nextSquare & allPlayerPieces).any()
                                            && castlingRook != nullptr
                                            && castlingRook->getOwner() == player.getPlayerNum()
                                            && !castlingRook->getMovedBefore()
                                            &&
                                            (castlingRook->getCharRep() == 'r' || castlingRook->getCharRep() == 'R')) {
                                            //Add the pseudo move
                                            if (returnSet.count(x.first) == 0) {
                                                returnSet.insert({x.first, std::unordered_set<Move>()});
                                            }
                                            returnSet.at(x.first).insert({false,
                                                                          castlingRook,
                                                                          true,
                                                                          false,
                                                                          {z.locationDelta.start, end}
                                                                         });
                                        }
                                    }
                                    break;
                                }
                            }
                            //QUEENSIDE castling
                            //Check if player can move one square queenside
                            //Find the move to the left, if it exists
                            for (auto &z:pseudoMoves.at(x.first)) {
                                //Make sure this is not a capture move
                                if (!z.capture
                                    && z.locationDelta.end.x - z.locationDelta.start.x == -1
                                    && z.locationDelta.end.y - z.locationDelta.start.y == 0) {
                                    if (isMoveValid(z, gameState, player.getPlayerNum(), board)) {
                                        //Check if the next square is not occupied

                                        Location end = z.locationDelta.end;
                                        end.x -= 1;
                                        boost::dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
                                        nextSquare.set(bitsetUtil::getIndex(board.getWidth(), end), true);

                                        //Make sure the square next to the rook is empty as well
                                        nextSquare.set(bitsetUtil::getIndex(board.getWidth(), {end.x - 1, end.y}),
                                                       true);


                                        Location castlingRookLoc = end;
                                        castlingRookLoc.x -= 2;
                                        std::shared_ptr<Piece> castlingRook = gameState.pieceAt(castlingRookLoc);
                                        if (!(nextSquare & allPlayerPieces).any()
                                            && castlingRook != nullptr
                                            && castlingRook->getOwner() == player.getPlayerNum()
                                            && !castlingRook->getMovedBefore()
                                            &&
                                            (castlingRook->getCharRep() == 'r' || castlingRook->getCharRep() == 'R')) {
                                            //Add the pseudo move
                                            if (returnSet.count(x.first) == 0) {
                                                returnSet.insert({x.first, std::unordered_set<Move>()});
                                            }
                                            returnSet.at(x.first).insert({false,
                                                                          castlingRook,
                                                                          false,
                                                                          true,
                                                                          {z.locationDelta.start, end}
                                                                         });
                                        }
                                    }
                                    break;
                                }
                            }

                            break; //king check
                        }
                    }
                }
                return returnSet;
            } //genpseudocastle
        } //Anon namespace


        std::map<boost::uuids::uuid, std::unordered_set<Move>>
        generatePseudoLegalMoves(GameState &gameState, Player &player, Board &board) {
            std::map<boost::uuids::uuid, std::unordered_set<Move>> returnMoves = {};

            //Calculate captures
            std::map<boost::uuids::uuid, std::unordered_set<Move>> captures = generatePseudoMoveCaptures_(gameState,
                                                                                                          true,
                                                                                                          player,
                                                                                                          board);
            for (auto &x:captures) {
                returnMoves.insert({x.first, x.second});
            }

            //Calculate translations
            std::map<boost::uuids::uuid, std::unordered_set<Move>> translates = generatePseudoMoveCaptures_(gameState,
                                                                                                            false,
                                                                                                            player,
                                                                                                            board);

            for (auto &x:translates) {
                if (returnMoves.count(x.first) == 0) {
                    returnMoves.insert({x.first, std::unordered_set<Move>()});
                }
                for (auto &y:x.second) {
                    returnMoves.at(x.first).insert(y);
                }
            }


            //Calculate castling moves
            std::map<boost::uuids::uuid, std::unordered_set<Move>> castles = generateCastlingPseudoMoves_(gameState,
                                                                                                          returnMoves,
                                                                                                          player,
                                                                                                          board);

            for (auto &x:castles) {
                if (returnMoves.count(x.first) == 0) {
                    returnMoves.insert({x.first, std::unordered_set<Move>()});
                }
                for (auto &y:x.second) {
                    returnMoves.at(x.first).insert(y);
                }
            }

            return returnMoves;
        }



        std::unordered_set<int> playersInCheck(GameState &gameState, Board &board) {
            std::unordered_set<int> playersInCheck = {};
            std::map<int, Player> &players = gameState.getPlayerMap();

            for (auto &x:players) {
                std::map<boost::uuids::uuid, std::unordered_set<Move>> captures =
                        generatePseudoMoveCaptures_(gameState, true, x.second, board);

                for (auto &y:captures) {
                    for (auto &z:y.second) {
                        //z.targetPiece
                        if (z.targetPiece->getAppliesCheck()) {
                            playersInCheck.insert(z.targetPiece->getOwner());
                        }
                    }
                }
            }
            return playersInCheck;
        }

        bool isMoveValid(const Move &move, GameState &gameState, int playerNum, Board &board) {
            bool returnVal = false;
            gameState.makeMove(move);
            std::unordered_set<int> playersInCheck = movegen::playersInCheck(gameState, board);
            if (playersInCheck.find(playerNum) == playersInCheck.end()) {
                //This move is ok; does not brings player into check
                returnVal = true;
            }
            gameState.unmakeMove(move);
            return returnVal;
        }
    }
}

