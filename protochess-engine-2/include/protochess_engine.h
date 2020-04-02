//
// Created by raytran on 3/31/20.
//
#pragma once
#include <string>
#include "../src/gamestate.h"

namespace protochess_engine{
    class ProtochessEngine {
    private:
        GameState gameState;
        const Dimensions dimensions;
        std::unordered_map<char, dynamic_bitset<>> charToPieces(std::vector<char> &pieces);

    public:
        ProtochessEngine(int width, int height);

        void fromFEN(std::string fenString);

        void getMoves(int playerNum);
        std::string toString();
    };
}
