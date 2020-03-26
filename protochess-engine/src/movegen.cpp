//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "movegen.h"
#include "bitsetUtil.h"
#include "piecerules.h"

using namespace bitsetUtil;
namespace movegen {
    namespace {
        std::map<boost::uuids::uuid, std::vector<Move>> generateMoves_(bool captures, Player &player, Board &board) {
            std::map<boost::uuids::uuid, std::vector<Move>> returnSet;

            std::map<boost::uuids::uuid, Piece> playerPieceMap = player.getPieces();
            std::map<char, MovementPattern> playerMPmap = captures ? player.getCaptureMap() : player.getMovementMap();
            boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());

            //Generate all of this player's pieces
            boost::dynamic_bitset<> thisPlayerPieces = player.getAllPiecesBitset();

            //generate enemies
            boost::dynamic_bitset<> enemyPieces;
            enemyPieces = allPlayerPieces & (~thisPlayerPieces);


            for (auto &x:playerPieceMap) {
                //Determine which movement pattern to use
                MovementPattern thisMP;
                if (playerMPmap.count(x.second.getCharRep()) != 0) {
                    thisMP = playerMPmap.at(x.second.getCharRep());
                } else {
                    std::cerr << "WARNING: unknown piece MP; defaulting to king-style movement\n";
                    thisMP = piecerules::moveRules.at('k');
                }

                //Squares that the piece can move to, including captures
                boost::dynamic_bitset<> validSquares(board.getWidth() * board.getHeight());
                int locIndex = x.second.getLocationIndex();
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
                    validSquares ^= bitsetUtil::translate(m, x.second.getBitset(), board);
                }

                //Filter out attacks on your own pieces
                validSquares &= ~thisPlayerPieces;

                //Convert the valid squares board to moves
                if (validSquares.any()) {
                    std::vector<LocationDelta> deltas = bitboardsToDeltas(board.getDimensions(), x.second.getBitset(),
                                                                          validSquares);
                    for (auto &delta : deltas) {
                        boost::dynamic_bitset<> singleEndPoint(board.getWidth() * board.getHeight());
                        singleEndPoint.set(bitsetUtil::getIndex(board.getWidth(), delta.end), true);
                        bool captureHere = (singleEndPoint & enemyPieces).any();
                        if (captures == captureHere) {
                            if (returnSet.count(x.first) == 0) {
                                returnSet.insert({x.first, std::vector<Move>()});
                            }
                            returnSet.at(x.first).push_back({captureHere, delta});
                        }
                    }
                }
            }
            return returnSet;
        }
    }

    std::map<boost::uuids::uuid, std::vector<Move>> generateMoves(Player &player, Board &board) {
        std::map<boost::uuids::uuid, std::vector<Move>> returnMoves;
        std::map<boost::uuids::uuid, std::vector<Move>> captures = generateMoves_(true, player, board);
        for (auto &x:captures) {
            returnMoves.insert({x.first, x.second});
        }
        std::map<boost::uuids::uuid, std::vector<Move>> translates = generateMoves_(false, player, board);
        for (auto &x:translates) {
            if (returnMoves.count(x.first) == 0) {
                returnMoves.insert({x.first, x.second});
            } else {
                for (auto &y:x.second) {
                    returnMoves.at(x.first).push_back(y);
                }
            }
        }
        return returnMoves;
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
}
