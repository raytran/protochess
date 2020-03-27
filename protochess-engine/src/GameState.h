//
// Created by raytran on 3/26/20.
//

#pragma once

#include <map>
#include <unordered_set>
#include "Board.h"

namespace protochess_engine {
    class GameState {
    private:
        Board board;
        std::map<int, Player> players;
        int playerCounter = 0;
        int whosTurn = 0;
        Dimensions dimensions;

    public:
        explicit GameState(Dimensions dim);

        //Dimension related
        const Dimensions &getDimensions();

        void setPieces(int playerNum, std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap);

        void setMovementMap(int playerNum, std::map<char, MovementPattern> moveMap);

        void setCaptureMap(int playerNum, std::map<char, MovementPattern> captureMap);

        std::map<int, Player> &getPlayerMap();

        int getWidth();

        int getHeight();

        std::string toString();

        int registerPlayer(std::string name);

        void incrementTurn();

        int getWhosTurn();

        //Unmakes a move, assuming its valid
        void unmakeMove(const Move &move);

        //Makes a move, assuming its valid
        void makeMove(const Move &move);

        //Returns the piece at this location
        std::shared_ptr<Piece> pieceAt(Location loc);

        //Generates the LEGAL moves for a player at player num
        std::map<boost::uuids::uuid, std::unordered_set<Move>> generateMoves(int playerNum);

        //Called after a move / after modifying properties
        void update();
    };


}
