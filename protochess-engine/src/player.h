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
        bool canCastle_ = true;

        int playerNum;

        std::string name;
        //MAPs char to a bitboard of that piece
        //Not using an enum here to allow for any arbitrary number of piece types
        boost::dynamic_bitset<> allPieces;
        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieces;
        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> piecesApplyCheck;
        //How this player defines each piece to move
        std::map<char, MovementPattern> movementMap;

        //How this player defines each piece to capture
        std::map<char, MovementPattern> captureMap;
    public:
        explicit Player(int playerNum);

        explicit Player(int playerNum, std::string name);

        std::string getName();

        void setPieces(std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap);

        void removePiece(boost::uuids::uuid pieceId);

        void addPiece(std::shared_ptr<Piece> piece);

        void update();

        int getPlayerNum();

        void disableCastleRights();

        void enableCastleRights();

        bool canCastle();

        void setMovementMap(std::map<char, MovementPattern> map);

        void setCaptureMap(std::map<char, MovementPattern> map);

        const std::map<char, MovementPattern> &getMovementMap() const;

        const std::map<char, MovementPattern> &getCaptureMap() const;

        const std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &getPieces() const;

        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &getPiecesApplyCheck();

        const boost::dynamic_bitset<> &getAllPiecesBitset();

        std::shared_ptr<Piece> getPieceAt(Location loc);

    };
}
