//
// Created by raytran on 4/1/20.
//

#pragma once
#include <string>
#include "../src/position.h"
#include "../src/attacktables.h"

namespace protochess_engine{
    class ProtochessEngine {
    private:
        Dimensions dimensions;
        std::unique_ptr<Position> currentPosition;
        std::unique_ptr<AttackTables> attackTables;
        int numPlayers = 2;
        unsigned long long perft_(int depth, int whosTurn);
    public:

        ProtochessEngine();

        //Assumes the FEN string is well formatted
        void loadFEN(std::string fenString);

        unsigned long long perft(int depth);

        void makeMove(Move m);

        void unmakeMove(Move m);

        const MovementPattern *getTranslatePattern(char c) const;

        const MovementPattern *getCapturePattern(char c) const;


        std::string toString();
    };
}
