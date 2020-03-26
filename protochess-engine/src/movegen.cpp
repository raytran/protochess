//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "movegen.h"
#include "bitsetUtil.h"
#include "moverules.h"

using namespace bitsetUtil;
namespace movegen {
    std::vector<Move> generateMoves(Player &player, Board &board) {
        std::vector<Move> returnSet;
        std::map<boost::uuids::uuid, Piece> playerPieceMap = player.getPieces();
        std::map<char, MovementPattern> playerMPmap = player.getMovementMap();
        boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());

        //Generate all of this player's pieces
        boost::dynamic_bitset<> thisPlayerPieces(board.getWidth() * board.getHeight());
        for (auto const &x:playerPieceMap) thisPlayerPieces ^= x.second.getBitset();

        //generate enemies
        boost::dynamic_bitset<> enemyPieces;
        enemyPieces = allPlayerPieces & (~thisPlayerPieces);

        std::cout << "ENEMIES:\n";
        std::cout << bitsetUtil::bitsetToString(enemyPieces, board.getDimensions());
        for (auto const &x:playerPieceMap) {
            //Determine which movement pattern to use
            MovementPattern thisMP;
            if (playerMPmap.count(x.second.getCharRep()) != 0) {
                thisMP = playerMPmap.at(x.second.getCharRep());
            } else {
                std::cerr << "WARNING: unknown piece MP; defaulting to king-style movement\n";
                thisMP = moverules::rules.at('k');
            }

            //Squares that the piece can move to, including captures
            boost::dynamic_bitset<> validSquares(board.getWidth() * board.getHeight());
            //SLIDING MOVES:
            //POSITIVE ATTACKS
            if (thisMP.north) {
                validSquares ^= calculatePositiveAttacks(NORTH, board,
                                                         board.getRayAttack(NORTH, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.east) {
                validSquares ^= calculatePositiveAttacks(EAST, board,
                                                         board.getRayAttack(EAST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.northEast) {
                validSquares ^= calculatePositiveAttacks(NORTHEAST, board,
                                                         board.getRayAttack(NORTHEAST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.northWest) {
                validSquares ^= calculatePositiveAttacks(NORTHWEST, board,
                                                         board.getRayAttack(NORTHWEST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }

            //NEGATIVE ATTACKS
            if (thisMP.south) {
                validSquares ^= calculateNegativeAttacks(SOUTH, board,
                                                         board.getRayAttack(SOUTH, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.west) {
                validSquares ^= calculateNegativeAttacks(WEST, board,
                                                         board.getRayAttack(WEST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.southEast) {
                validSquares ^= calculateNegativeAttacks(SOUTHEAST, board,
                                                         board.getRayAttack(SOUTHEAST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }
            if (thisMP.southWest) {
                validSquares ^= calculateNegativeAttacks(SOUTHWEST, board,
                                                         board.getRayAttack(SOUTHWEST, x.second.getLocationIndex()),
                                                         allPlayerPieces);
            }

            //NONSLIDING PIECES
            for (auto const &m : thisMP.deltas) {
                validSquares ^= bitsetUtil::translate(m, x.second.getBitset(), board);
            }

            //Filter out attacks on your own pieces
            validSquares &= ~thisPlayerPieces;

            //Convert the valid squares board to moves
            std::vector<LocationDelta> deltas = bitboardsToDeltas(board.getDimensions(), x.second.getBitset(),
                                                                  validSquares);
            for (auto &delta : deltas) {
                boost::dynamic_bitset<> singleEndPoint(board.getWidth() * board.getHeight());
                singleEndPoint.set(bitsetUtil::getIndex(board.getWidth(), delta.end), true);
                bool captureHere = (singleEndPoint & enemyPieces).any();
                returnSet.push_back({captureHere, delta});
            }
        }
        return returnSet;
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
