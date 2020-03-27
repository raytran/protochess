//
// Created by raytran on 3/24/20.
//

#pragma once

#include <string>
#include <map>
#include "types.h"
#include "piece.h"
#include <boost/dynamic_bitset.hpp>

namespace protochess_engine {
    class Player {
    private:
        std::string name;
        //MAPs char to a bitboard of that piece
        //Not using an enum here to allow for any arbitrary number of piece types
        boost::dynamic_bitset<> allPieces;
        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieces;
        //How this player defines each piece to move
        std::map<char, MovementPattern> movementMap;

        //How this player defines each piece to capture
        std::map<char, MovementPattern> captureMap;
    public:
        Player();

        explicit Player(std::string name);

        std::string getName();

        void setPieces(std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap);

        void removePiece(boost::uuids::uuid pieceId);

        void addPiece(const std::shared_ptr<Piece> &piece);

        void update();

        void setMovementMap(std::map<char, MovementPattern> map);

        void setCaptureMap(std::map<char, MovementPattern> map);

        std::map<char, MovementPattern> &getMovementMap();

        std::map<char, MovementPattern> &getCaptureMap();

        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &getPieces();

        boost::dynamic_bitset<> getAllPiecesBitset();

        boost::uuids::uuid getPieceIdAt(Location loc);

    };
}
