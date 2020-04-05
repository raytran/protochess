//
// Created by raytran on 4/1/20.
//
#pragma once
//Represents a board position in the game
#include <unordered_map>
#include "types.h"
#include "bitboard.h"

namespace protochess_engine {
    class Position {
    private:
        const Dimensions dimensions;
        //Maps playerNum -> map of char, bitboard
        std::vector<std::unordered_map<char, bitboard>> playerPiecesMap;
        //playerNum->allpiecesBB
        std::vector<bitboard> playerAllPieces;
        //All pieces on the board
        bitboard allPieces;

        void update();

    public:

        Position(Dimensions dimensions, std::vector<std::unordered_map<char, bitboard>> startingPieces);


        // equality operator that helps with: if (mDate1 == mDate2)...
        bool operator==(const Position &rhs);

        // inequality operator
        bool operator!=(const Position &rhs);

        std::string toString();

        void makeMove(const Move &move);

        void unmakeMove(const Move &move);

        Piece pieceAt(int x, int y) const;

        Piece pieceAt(int index) const;


        [[nodiscard]] const Dimensions &getDimensions() const;

        [[nodiscard]] const std::vector<std::unordered_map<char, bitboard>> &getPlayerPiecesMap() const;

        [[nodiscard]] const std::vector<bitboard> &getPlayerAllPieces() const;

        [[nodiscard]] const bitboard &getAllPieces() const;

    };
}
