//
// Created by raytran on 4/1/20.
//
#pragma once
#include "bitboardutil.h"
#include "types.h"
#include "attacktables.h"

namespace protochess_engine::bitboardutil {
    bitboard east(int amt, const bitboard &in, const AttackTables *tables);

    bitboard west(int amt, const bitboard &in, const AttackTables *tables);

    bitboard north(int amt, const bitboard &in, const AttackTables *tables);

    bitboard south(int amt, const bitboard &in, const AttackTables *tables);

    bitboard translate(const Location &delta, const bitboard &in, const AttackTables *attackTables);

    std::string bitsetToString(bitboard bitboard, const Dimensions &dimensions);

    int getIndex(int width, int x, int y);

    Location getLoc(int width, int index);
}


