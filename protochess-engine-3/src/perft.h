//
// Created by raytran on 4/5/20.
//
#pragma once

#include "../include/protochess_engine.h"

namespace protochess_engine::perft_tests {
    unsigned long long
    perft(int depth, ProtochessEngine *engine, AttackTables *tables, Rulebook *rulebook, Position *position);

    unsigned long long
    perft_divide(int depth, ProtochessEngine *engine, AttackTables *tables, Rulebook *rulebook, Position *position);
}
