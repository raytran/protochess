//
// Created by raytran on 3/31/20.
//

#include "../include/protochess_engine.h"
#include "bitsetutil.h"
#include "piecerules.h"
#include <iostream>

namespace protochess_engine{
    ProtochessEngine::ProtochessEngine(int width, int height):
            dimensions({width,height}),
            gameState({width,height}){
        std::vector<char> wPieces = {
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P',
                'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'
        };

        std::vector<char> bPieces = {
                'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r',
                'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '
        };

        gameState.addPlayer(charToPieces(wPieces));
        gameState.addPlayer(charToPieces(bPieces));

        //Initialize the game's movement/capture rules
        gameState.setMovementRules(piecerules::moveRules);
        gameState.setCaptureRules(piecerules::captureRules);
    }

    void ProtochessEngine::fromFEN(std::string fenString) {

    }

    std::string ProtochessEngine::toString() {
        return gameState.toString();
    }


    std::unordered_map<char, dynamic_bitset<>> ProtochessEngine::charToPieces(std::vector<char> &pieces) {
        std::unordered_map<char, dynamic_bitset<>> returnPieces;
        if (pieces.size() != dimensions.width * dimensions.height) {
            throw std::runtime_error("ERROR: invalid Piece array size");
        }

        int numBits = dimensions.width * dimensions.height;
        int index = 0;
        for (int y = dimensions.height - 1; y >= 0; y--) {
            for (int x = 0; x < dimensions.width; x++) {
                if (pieces[index] != ' ') {
                    char charHere = pieces[index];
                    if (!returnPieces.contains(charHere)){
                        returnPieces.insert({charHere,dynamic_bitset<>(numBits)});
                    }
                    returnPieces.at(charHere).set(bitsetUtil::getIndex(dimensions.width,{x,y}),true);
                }
                index++;
            }
        }
        return returnPieces;
    }

    void ProtochessEngine::getMoves(int playerNum) {
        std::cout<< gameState.getMoves(playerNum).size();
    }
}
