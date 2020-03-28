//
// Created by raytran on 3/25/20.
//

#pragma once


#include <boost/dynamic_bitset.hpp>
#include <unordered_set>
#include "../include/shared/chess.h"
#include "types.h"
#include "player.h"
#include "board.h"

namespace protochess_engine {
    namespace movegen {
        bool isMoveValid(const Move &move, GameState &gameState, int playerNum, Board &board);

        std::unordered_set<int> playersInCheck(GameState &gameState, Board &board);

        std::map<boost::uuids::uuid, std::unordered_set<Move>>
        generatePseudoLegalMoves(GameState &gameState, Player &player, Board &board);
    }
}


