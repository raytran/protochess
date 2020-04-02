//
// Created by raytran on 4/1/20.
//

#pragma once

#include "types.h"
#include "position.h"
#include "../include/protochess_engine.h"

namespace protochess_engine::movegen{
    std::vector<Move> generateLegalMoves(const ProtochessEngine *engine, const AttackTables *tables, int whosTurn, const Position *position);

};

