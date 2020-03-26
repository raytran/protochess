//
// Created by raytran on 3/25/20.
//

#pragma once


#include <boost/dynamic_bitset.hpp>
#include <unordered_set>
#include "types.h"
#include "player.h"
#include "Board.h"

namespace movegen {
    //Calculates north attacks, stopping (and including) the first blocker
    //Does not filter own pieces
    boost::dynamic_bitset<>
    calculatePositiveAttacks(const Direction &dir, Board &board, const boost::dynamic_bitset<> &northAttack,
                             const boost::dynamic_bitset<> &allPieces);

    boost::dynamic_bitset<>
    calculateNegativeAttacks(const Direction &dir, Board &board, const boost::dynamic_bitset<> &southAttack,
                             const boost::dynamic_bitset<> &allPieces);

    std::map<boost::uuids::uuid, std::vector<Move>> generateMoves(Player &player, Board &board);

    //Converts a bitboard w/one piece and its bitboard with destinations squares
    //to a set of location deltas
    std::vector<LocationDelta> bitboardsToDeltas(const Dimensions &dimensions, const boost::dynamic_bitset<> &onePiece,
                                                 boost::dynamic_bitset<> destinations);
}


