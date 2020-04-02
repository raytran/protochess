//
// Created by raytran on 4/1/20.
//

#include "position.h"

#include <utility>
#include "types.h"
#include "bitboardutil.h"

namespace protochess_engine{
    Position::Position(Dimensions dimensions, std::vector<std::unordered_map<char, bitboard>> startingPieces):
            playerPiecesMap(std::move(startingPieces)),
            playerAllPieces(playerPiecesMap.size(), 0),
            dimensions({dimensions.width,dimensions.height}) {
        //Update the internal definitions
        update();
    }

    std::string Position::toString() {
        std::string returnString;
        int width = dimensions.width;
        int height = dimensions.height;
        for (int y = height - 1; y >= 0; y--) {
            for (int x = 0; x < width; x++) {
                Piece piece = pieceAt(x,y);
                if (piece.charType != ' ') {
                    returnString += piece.charType;
                    returnString += " ";
                } else {
                    returnString += ". ";
                }
            }
            returnString += "\n";
        }

        return returnString;
    }


    void Position::update(){
        allPieces = 0;
        for (int i=0; i < playerPiecesMap.size(); i++){
            playerAllPieces[i] = 0;
            for (auto &z:playerPiecesMap[i]){
                playerAllPieces[i] |= z.second;
                allPieces |= z.second;
            }
        }
    };


    const Dimensions &Position::getDimensions() const{
        return dimensions;
    }
    const std::vector<std::unordered_map<char,bitboard>> &Position::getPlayerPiecesMap() const{
        return playerPiecesMap;
    }
    const std::vector<bitboard> &Position::getPlayerAllPieces() const{
        return playerAllPieces;
    }
    const bitboard &Position::getAllPieces() const{
        return allPieces;
    }

    Piece Position::pieceAt(int index) const {
        if (!boost::multiprecision::bit_test(allPieces,index)){
            return {-1,' '};
        }
        for (int i=0;i<playerPiecesMap.size();i++){
            for (auto &z:playerPiecesMap[i]){
                if (boost::multiprecision::bit_test(z.second,index)){
                    return {i,z.first};
                }
            }
        }
        throw std::runtime_error("Piece map not updated with allPieces");
    }


    Piece Position::pieceAt(int x, int y) const {
        return pieceAt(bitboardutil::getIndex(dimensions.width,x,y));
    }

    void Position::makeMove(Move &move) {
        int fromIndex = move.getFrom();
        int toIndex = move.getTo();
        Piece sourcePiece = pieceAt(fromIndex);
        if (sourcePiece.owner != -1){
            if (move.getIsCapture()){
                //Handle capture
                Piece capturedPiece = move.getCapturedPiece();
                bitboard &captured = playerPiecesMap[capturedPiece.owner].at(capturedPiece.charType);
                bit_unset(captured,toIndex);
            }

            //Move the piece
            bitboard &pieces = playerPiecesMap[sourcePiece.owner].at(sourcePiece.charType);
            bit_unset(pieces,fromIndex);
            bit_set(pieces,toIndex);

        }else{
            Location from = bitboardutil::getLoc(8,fromIndex);
            Location to = bitboardutil::getLoc(8,toIndex);
            std::cerr<<from.x;
            std::cerr<<from.y;
            std::cerr<<'-';
            std::cerr<<to.x;
            std::cerr<<to.y;
            std::cerr<<"MAKEMOVE No piece here.\n";
        }

        update();
    }

    void Position::unmakeMove(Move &move) {
        //Swap from and no
        int fromIndex = move.getTo();
        int toIndex = move.getFrom();
        Piece sourcePiece = pieceAt(fromIndex);
        if (sourcePiece.owner != -1){
            //Move the piece
            bitboard &pieces = playerPiecesMap[sourcePiece.owner].at(sourcePiece.charType);
            bit_unset(pieces,fromIndex);
            bit_set(pieces,toIndex);

            if (move.getIsCapture()){
                //Undo capture
                Piece capturedPiece = move.getCapturedPiece();
                bitboard &captured = playerPiecesMap[capturedPiece.owner].at(capturedPiece.charType);

                //Remember: fromIndex is relative to source piece
                bit_set(captured,fromIndex);

            }

        }else{
            Location from = bitboardutil::getLoc(8,fromIndex);
            Location to = bitboardutil::getLoc(8,toIndex);
            std::cerr<<from.x;
            std::cerr<<from.y;
            std::cerr<<'-';
            std::cerr<<to.x;
            std::cerr<<to.y;
            std::cerr<<"UNMAKE No piece here.\n";
        }

        update();
    }

    bool Position::operator==(const Position &rhs) {
        return (dimensions.width == rhs.dimensions.width
                && dimensions.height == rhs.dimensions.height
                && playerPiecesMap == rhs.playerPiecesMap
                && playerAllPieces == rhs.playerAllPieces
                && allPieces == rhs.allPieces);
    }

    bool Position::operator!=(const Position &rhs) {
        return (dimensions.width != rhs.dimensions.width
                || dimensions.height != rhs.dimensions.height
                || playerPiecesMap != rhs.playerPiecesMap
                || playerAllPieces != rhs.playerAllPieces
                || allPieces != rhs.allPieces);
    }
}



