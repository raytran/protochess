#pragma once

#include <string>
#include <boost/dynamic_bitset.hpp>
#include "../../src/types.h"
#include "../../src/player.h"
#include "../../src/Board.h"

class Chess {
private:
    Board board;
    std::map<int, Player> players;
    int playerCounter = 0;
    int whosTurn = 0;
    Dimensions dimensions;

    //Called after a move
    void update();

    Piece &pieceAt(Location loc);

    //Makes a move, assuming its valid
    void makeMove(Move move);

    //Converts a char array to piece , assigning each piece a uuid at the same time
    std::map<boost::uuids::uuid, Piece> charToPieces(int owner, const char *pieces);

    //Converts a char array to associated movement patterns declared in source
    std::map<char, MovementPattern> charToKnownMP(std::map<char, MovementPattern> &dictionary, const char *pieces);


public:
    Chess();

    Chess(int width, int height);

    bool takeTurn(int startX, int startY, int endX, int endY);

    int registerPlayer(std::string playerName);

    void buildClassicSet();

    std::string toString();

    void reset();
};

