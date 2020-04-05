//
// Created by raytran on 4/1/20.
//

#pragma once
#include <string>
#include "../src/position.h"
#include "../src/attacktables.h"
#include "../src/rulebook.h"

namespace protochess_engine{
    class ProtochessEngine {
    private:
        Dimensions dimensions;
        int whosTurn = 0;
        std::unique_ptr<Position> currentPosition;
        std::unique_ptr<AttackTables> attackTables;
        std::unique_ptr<Rulebook> pieceRulebook;
        int numPlayers = 2;

        unsigned long long perft_(int depth, int whosTurn);

        unsigned long long perft_fast_(int depth, int whosTurn);

        unsigned long long perft_divide_(int depth, int whosTurn);

    public:

        ProtochessEngine();

        //Assumes the FEN string is well formatted
        void loadFEN(std::string fenString);

        unsigned long long perft(int depth);

        unsigned long long perft_divide(int depth);

        bool makeMove(std::string from, std::string to);

        int getWhosTurn();

        int getNumPlayers();



        std::string toString();
    };
}
