//
// Created by raytran on 3/31/20.
//
#pragma once

#include <string>
#include "types.h"
#include "attacktables.h"
#include "movegen.h"

using boost::dynamic_bitset;
using std::vector;
using std::unordered_map;
using std::shared_ptr;
namespace protochess_engine{
    class GameState {
    private:
        const Dimensions dimensions;
        const AttackTables attackTables;
        friend class MoveGenerator;
        const MoveGenerator moveGenerator;
        dynamic_bitset<> allPieces;

        vector<unordered_map<char, dynamic_bitset<>>> initialPlayerPieces;
        vector<unordered_map<char, dynamic_bitset<>>> playerPieces;
        vector<dynamic_bitset<>> playerPiecesAll;

        std::unordered_map<char, MovementPattern> moveRules;
        std::unordered_map<char, MovementPattern> captureRules;

        int whosTurn = 0;

        void update();
    public:
        explicit GameState(Dimensions dimensions);

        void addPlayer(unordered_map<char,dynamic_bitset<>> startingPieces);

        void makeMove();

        void unmakeMove();

        std::string toString();

        void setCaptureRules(std::unordered_map<char,MovementPattern> captureRules);

        void setMovementRules(std::unordered_map<char,MovementPattern> moveRules);

        char pieceTypeAt(const Location &loc);

        std::unordered_set<Move> getMoves(int playerNum);
    };
}
