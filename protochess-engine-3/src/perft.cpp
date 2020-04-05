//
// Created by raytran on 4/5/20.
//

#include "perft.h"
#include "movegen.h"
#include "bitboardutil.h"
#include "rankfile.h"

namespace protochess_engine::perft_tests {
    namespace {
        unsigned long long perft_(int depth, int whosTurn,
                                  ProtochessEngine *engine,
                                  AttackTables *tables,
                                  Rulebook *rulebook,
                                  Position *position) {
            if (depth == 0) return 1;
            unsigned long long nodes = 0;
            std::vector<Move> moves = movegen::generateLegalMoves(engine,
                                                                  tables,
                                                                  rulebook,
                                                                  whosTurn,
                                                                  position);
            if (depth == 1) return moves.size();
            for (auto &move : moves) {
                position->makeMove(move);
                nodes += perft_(depth - 1,
                                whosTurn + 1 % engine->getNumPlayers(),
                                engine,
                                tables,
                                rulebook,
                                position);
                position->unmakeMove(move);
            }
            return nodes;
        }
    }

    unsigned long long perft(int depth, ProtochessEngine *engine, AttackTables *tables,
                             Rulebook *rulebook, Position *position) {
        return perft_(depth, engine->getWhosTurn(), engine, tables, rulebook, position);
    }

    unsigned long long
    perft_divide(int depth, ProtochessEngine *engine, AttackTables *tables, Rulebook *rulebook, Position *position) {
        unsigned long long nodes = 0;

        std::vector<Move> moves = movegen::generateLegalMoves(engine,
                                                              tables,
                                                              rulebook,
                                                              engine->getWhosTurn(),
                                                              position);
        for (auto &move : moves) {
            position->makeMove(move);

            unsigned long long additional = perft_(depth - 1,
                                                   engine->getWhosTurn() + 1 % engine->getNumPlayers(),
                                                   engine,
                                                   tables,
                                                   rulebook,
                                                   position);
            Location from = bitboardutil::getLoc(8, move.getFrom());
            Location to = bitboardutil::getLoc(8, move.getTo());
            std::cout << position->pieceAt(from.x, from.y).charType;
            std::cout << rankfile::toRankFile(from);
            std::cout << "-";
            std::cout << rankfile::toRankFile(to);
            std::cout << " :";
            std::cout << additional;
            std::cout << "\n";
            nodes += additional;
            position->unmakeMove(move);
        }
        return nodes;
    }
}
