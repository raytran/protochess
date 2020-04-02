//
// Created by raytran on 4/1/20.
//

#pragma once
#include "types.h"
#include <iostream>
namespace protochess_engine {
    class GameState;
    class MoveGenerator{
    private:
        GameState &gameState;
    public:
        explicit MoveGenerator(GameState &gameState);
        std::unordered_set<Move> generatePseudoLegalMoves(int whosTurn) const;
    };
};


