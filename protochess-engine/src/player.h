//
// Created by raytran on 3/24/20.
//

#pragma once

#include <string>
#include <map>
#include "types.h"
#include "Piece.h"
#include <boost/dynamic_bitset.hpp>

class Player {
private:
    std::string name;
    //MAPs char to a bitboard of that piece
    //Not using an enum here to allow for any arbitrary number of piece types
    std::map<boost::uuids::uuid, Piece> pieces;
    //How this player defines each piece to move
    std::map<char, MovementPattern> movementMap;
public:
    Player();

    explicit Player(std::string name);

    std::string getName();

    void setPieces(std::map<boost::uuids::uuid, Piece> pieceMap);

    void setMovementMap(std::map<char, MovementPattern> map);

    const std::map<boost::uuids::uuid, Piece> &getPieces();

    const std::map<char, MovementPattern> &getMovementMap();
};


