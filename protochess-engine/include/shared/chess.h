#pragma once

#include <string>
#include <boost/dynamic_bitset.hpp>
#include "../../src/types.h"
#include "../../src/gamestate.h"

namespace protochess_engine {
    struct TurnResult {
        bool successful;
        int nextToMove;
        std::unordered_set<int> checkedPlayers;
        std::unordered_set<int> checkmatedPlayers;
    };

    class Chess {
    private:
        GameState gameState;

        //Converts a char array to piece , assigning each piece a uuid at the same time
        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> charToPieces(int owner, std::vector<char> &pieces);

    public:
        TurnResult takeTurn(const std::string &start, const std::string &end, int whosTurn);

        TurnResult takeTurn(int startX, int startY, int endX, int endY, int whosTurn);

        int registerPlayer(std::string playerName);

        void buildClassicSet();

        std::string toString();

        void reset();

        Chess();
    };

}
