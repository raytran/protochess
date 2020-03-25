//
// Created by raytran on 3/25/20.
//

#pragma once


#include <boost/dynamic_bitset.hpp>
#include "types.h"
#include "player.h"
#include "Board.h"

namespace movegen {
    std::set<Move> generateMoves(const Player &player, Board &board);
}


