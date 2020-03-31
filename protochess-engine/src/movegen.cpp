//
// Created by raytran on 3/25/20.
//

#include "movegen.h"
#include <iostream>
#include "bitsetutil.h"
#include "piecerules.h"
#include "types.h"
#include <chrono>

namespace protochess_engine {
    namespace movegen {
        using namespace bitsetUtil;
        using boost::dynamic_bitset;
        using boost::uuids::uuid;
        using std::map;
        using std::unordered_set;
        namespace {
            dynamic_bitset<> calculatePositiveAttacks(const Direction &dir,
                                                      const Board &board,
                                                      const dynamic_bitset<> &positiveAttack,
                                                      const dynamic_bitset<> &allPieces) {
                dynamic_bitset<> returnAttack(positiveAttack);
                dynamic_bitset<> blockers = positiveAttack & allPieces;
                if (blockers.any()) {
                    //Piece is being blocked
                    returnAttack = positiveAttack ^ board.getRayAttack(dir, blockers.find_first());
                }
                return returnAttack;
            }

            dynamic_bitset<> calculateNegativeAttacks(const Direction &dir,
                                                      const Board &board,
                                                      const dynamic_bitset<> &negativeAttack,
                                                      const dynamic_bitset<> &allPieces) {
                dynamic_bitset<> returnAttack(negativeAttack);
                dynamic_bitset<> blockers = negativeAttack & allPieces;
                if (blockers.any()) {
                    //Piece is being blocked
                    returnAttack = negativeAttack ^ board.getRayAttack(dir, bitsetUtil::findLast(blockers));
                }
                return returnAttack;
            }

            std::vector<LocationDelta> bitboardsToDeltas(const Dimensions &dimensions,
                                                         const dynamic_bitset<> &onePiece,
                                                         dynamic_bitset<> destinations) {

                std::vector<LocationDelta> returnSet;
                Location start = bitsetUtil::getLoc(dimensions.width, onePiece.find_first());

                while (destinations.find_first() != dynamic_bitset<>::npos) {
                    size_type index = destinations.find_first();
                    Location end = bitsetUtil::getLoc(dimensions.width, index);
                    returnSet.push_back({start, end});
                    destinations.set(index, false);
                }
                return returnSet;
            }

            dynamic_bitset<> generateValidSquares(const std::shared_ptr<Piece> &piece,
                                                  const MovementPattern &thisMP,
                                                  const Board &board,
                                                  const dynamic_bitset<> &allPlayerPieces,
                                                  const dynamic_bitset<> &thisPlayerPieces) {
                //Determine which movement pattern to use

                //Squares that the piece can move to, including captures
                dynamic_bitset<> validSquares(board.getWidth() * board.getHeight());
                int locIndex = piece->getLocationIndex();
                //SLIDING MOVES:
                //POSITIVE ATTACKS
                if (thisMP.north) {
                    validSquares |= calculatePositiveAttacks(NORTH, board,
                                                             board.getRayAttack(NORTH, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.east) {
                    validSquares |= calculatePositiveAttacks(EAST, board,
                                                             board.getRayAttack(EAST, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.northEast) {
                    validSquares |= calculatePositiveAttacks(NORTHEAST, board,
                                                             board.getRayAttack(NORTHEAST, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.northWest) {
                    validSquares |= calculatePositiveAttacks(NORTHWEST, board,
                                                             board.getRayAttack(NORTHWEST, locIndex),
                                                             allPlayerPieces);
                }

                //NEGATIVE ATTACKS
                if (thisMP.south) {
                    validSquares |= calculateNegativeAttacks(SOUTH, board,
                                                             board.getRayAttack(SOUTH, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.west) {
                    validSquares |= calculateNegativeAttacks(WEST, board,
                                                             board.getRayAttack(WEST, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.southEast) {
                    validSquares |= calculateNegativeAttacks(SOUTHEAST, board,
                                                             board.getRayAttack(SOUTHEAST, locIndex),
                                                             allPlayerPieces);
                }
                if (thisMP.southWest) {
                    validSquares |= calculateNegativeAttacks(SOUTHWEST, board,
                                                             board.getRayAttack(SOUTHWEST, locIndex),
                                                             allPlayerPieces);
                }

                //NONSLIDING PIECES
                for (auto &m : thisMP.deltas) {
                    //Edge case pawn movement
                    if ((piece->getCharRep() == 'p' || piece->getCharRep() == 'P')
                        && piece->getMovedBefore()) {
                        if (m.y == 2 || m.y == -2) {
                            continue;
                        }
                    }
                    validSquares |= bitsetUtil::translate(m, piece->getBitset(), board);
                }

                //Filter out attacks/moves on your own pieces
                validSquares &= ~thisPlayerPieces;

                return validSquares;
            }

            //Bitset returned is the locations of every destination for this player this turn
            //Inserts into returnSet
            void generatePseudoMoveCaptures_(map<uuid, unordered_set<Move>> &returnSet,
                                             GameState &gameState,
                                             bool captures,
                                             Player &player,
                                             Board &board) {

                dynamic_bitset<> return_bits(board.getWidth() * board.getHeight());
                const map<uuid, std::shared_ptr<Piece>> &playerPieceMap = player.getPieces();
                const map<char, MovementPattern> &playerMPmap = captures ? player.getCaptureMap()
                                                                         : player.getMovementMap();
                const dynamic_bitset<> &allPlayerPieces = board.getAllPieces();

                //Generate all of this player's pieces
                const dynamic_bitset<> &thisPlayerPieces = player.getAllPiecesBitset();

                //generate enemies
                dynamic_bitset<> enemyPieces = allPlayerPieces & (~thisPlayerPieces);


                for (auto &x:playerPieceMap) {
                    //Determine which movement pattern to use
                    MovementPattern &thisMP = piecerules::moveRules.at('k');
                    if (playerMPmap.count(x.second->getCharRep()) != 0) {
                        thisMP = playerMPmap.at(x.second->getCharRep());
                    } else {
                        std::cerr << x.second->getCharRep();
                        std::cerr << "WARNING: unknown piece MP; defaulting to king-style movement\n";
                    }

                    //Squares that the piece can move to, including captures
                    dynamic_bitset<> validSquares = generateValidSquares(x.second,
                                                                         thisMP,
                                                                         board,
                                                                         allPlayerPieces,
                                                                         thisPlayerPieces);

                    //Filter valid squares according to whether we want captures only
                    //Or non-captures only
                    if (captures) {
                        validSquares &= enemyPieces;
                    } else {
                        validSquares &= ~enemyPieces;
                    }

                    //Convert the valid squares board to moves
                    if (validSquares.any()) {
                        std::vector<LocationDelta> deltas = bitboardsToDeltas(board.getDimensions(),
                                                                              x.second->getBitset(),
                                                                              validSquares);
                        for (auto &delta : deltas) {
                            bool promotion = false;
                            char promoteTo = ' ';
                            //Promotion
                            if ((delta.end.y == 0 || delta.end.y == board.getHeight() - 1) &&
                                x.second->getPromotable()) {
                                promotion = true;
                                promoteTo = x.second->getPromoteTo();
                            }

                            if (returnSet.count(x.first) == 0) {
                                returnSet.insert({x.first, unordered_set<Move>()});
                            }
                            returnSet.at(x.first).insert(
                                    {x.second,
                                     promotion,
                                     promoteTo,
                                     captures,
                                     gameState.pieceAt(delta.end),
                                     false,
                                     false,
                                     delta}
                            );
                        }
                    }
                }
            }


            void generateCastlingPseudoMoves_(map<uuid, unordered_set<Move>> &returnSet,
                                              GameState &gameState,
                                              map<uuid, unordered_set<Move>> &pseudoMoves,
                                              Player &player,
                                              Board &board) {

                //Check if this player can castle
                if (player.canCastle()) {
                    dynamic_bitset<> allPlayerPieces = dynamic_bitset<>(board.getAllPieces());
                    //Generate all of this player's pieces
                    dynamic_bitset<> thisPlayerPieces = player.getAllPiecesBitset();
                    //generate enemies
                    dynamic_bitset<> enemyPieces;
                    enemyPieces = allPlayerPieces & (~thisPlayerPieces);
                    //Find the pieces that can apply check
                    for (auto &x:player.getPiecesApplyCheck()) {
                        //Check if this piece is a king that hasn't moved before
                        //And has ANY valid moves
                        if (pseudoMoves.find(x.first) != pseudoMoves.end()
                            && !x.second->getMovedBefore()) {
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
                                        dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
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
                                                returnSet.insert({x.first, unordered_set<Move>()});
                                            }
                                            returnSet.at(x.first).insert({z.sourcePiece,
                                                                          false,
                                                                          ' ',
                                                                          false,
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
                                        dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
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
                                                returnSet.insert({x.first, unordered_set<Move>()});
                                            }
                                            returnSet.at(x.first).insert({z.sourcePiece,
                                                                          false,
                                                                          ' ',
                                                                          false,
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
            } //genpseudocastle
        } //Anon namespace


        map<uuid, unordered_set<Move>>
        generatePseudoLegalMoves(GameState &gameState, Player &player, Board &board) {
            map<uuid, unordered_set<Move>> returnMoves = {};
            //Calculate captures
            generatePseudoMoveCaptures_(returnMoves,
                                        gameState,
                                        true,
                                        player,
                                        board);

            //Calculate translations
            generatePseudoMoveCaptures_(returnMoves,
                                        gameState,
                                        false,
                                        player,
                                        board);

            //Calculate castling moves
            generateCastlingPseudoMoves_(returnMoves,
                                         gameState,
                                         returnMoves,
                                         player,
                                         board);

            return returnMoves;
        }


        unordered_set<int> playersInCheck(GameState &gameState, Board &board) {
            unordered_set<int> playersInCheck = {};
            map<int, Player> &players = gameState.getPlayerMap();

            for (auto &x:players) {
                map<uuid, unordered_set<Move>> captures;
                generatePseudoMoveCaptures_(captures, gameState, true, x.second, board);

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
            bool returnVal = true;
            gameState.makeMove(move);
            for (auto &x:gameState.getPlayerMap()) {
                //Calculate moves for other players
                if (x.second.getPlayerNum() != playerNum) {
                    map<uuid, unordered_set<Move>> captures;
                    generatePseudoMoveCaptures_(captures, gameState, true, x.second, board);

                    for (auto &y:captures) {
                        for (auto &z:y.second) {
                            //z.targetPiece
                            if (z.targetPiece->getOwner() == playerNum && z.targetPiece->getAppliesCheck()) {
                                returnVal = false;
                                break;
                            }
                        }
                        if (!returnVal) break;
                    }
                    if (!returnVal) break;
                }
            }
            gameState.unmakeMove(move);
            return returnVal;
        }
    }
}

