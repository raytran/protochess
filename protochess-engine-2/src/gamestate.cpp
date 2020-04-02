//
// Created by raytran on 3/31/20.
//

#include "gamestate.h"
#include "bitsetutil.h"
#include <iostream>
#include <utility>
using boost::dynamic_bitset;
namespace protochess_engine{
    GameState::GameState(protochess_engine::Dimensions dims):
            dimensions(dims),
            attackTables(dims),
            allPieces(dims.width * dims.height),
            moveGenerator(*this){
    }
    std::string GameState::toString() {
        std::string returnString;
        int width = dimensions.width;
        int height = dimensions.height;
        for (int y = height - 1; y >= 0; y--) {
            for (int x = 0; x < width; x++) {
                char here = pieceTypeAt({x,y});
                if (here != ' ') {
                    returnString += here;
                    returnString += " ";
                } else {
                    returnString += ". ";
                }
            }
            returnString += "\n";
        }
        return returnString;
    }

    void GameState::makeMove() {

    }

    void GameState::unmakeMove() {

    }

    char GameState::pieceTypeAt(const Location &loc) {
        //There's a piece here!
        if ((allPieces & bitsetUtil::toBitset(loc,dimensions)).any()){
            for (auto &x:playerPieces){
                for (auto &y:x){
                    //Found it
                    if (y.second[bitsetUtil::getIndex(dimensions.width,loc)]){
                        return y.first;
                    }
                }
            }
            throw std::runtime_error("all pieces not updated with playerPieces");
        }else{
            return ' ';
        }
    }

    void GameState::update() {
        allPieces.reset();
        for (int i=0;i<playerPieces.size();i++){
            //Reset
            playerPiecesAll[i].reset();
            for (auto &x:playerPieces[i]){
                playerPiecesAll[i] |= x.second;
            }
        }

        for (const auto & i : playerPiecesAll){
            allPieces |= i;
        }
    }

    void GameState::addPlayer(unordered_map<char, dynamic_bitset<>> startingPieces) {
        playerPiecesAll.emplace_back(dimensions.width * dimensions.height);
        playerPieces.push_back(startingPieces);
        initialPlayerPieces.push_back(startingPieces);
        update();
    }

    void GameState::setMovementRules(std::unordered_map<char, MovementPattern> rules) {
        moveRules = std::move(rules);

    }

    void GameState::setCaptureRules(std::unordered_map<char, MovementPattern> rules) {
        captureRules = std::move(rules);
    }

    std::unordered_set<Move> GameState::getMoves(int playerNum) {
        return moveGenerator.generatePseudoLegalMoves(playerNum);
    }
}
