//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "movegen.h"
#include "bitsetUtil.h"
#include "moverules.h"

using namespace bitsetUtil;
namespace movegen {
    std::set<Move> generateMoves(const Player &player, Board &board) {
        std::set<Move> returnSet;
        std::map<char, boost::dynamic_bitset<>> pPieces = player.getPieces();
        boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());
        allPlayerPieces.reset();
        for (auto const &x:pPieces) {
            allPlayerPieces |= x.second;
            MovementPattern thisMP;

            if (thisMP.north) {

            }
            if (thisMP.east) {

            }
            if (thisMP.south) {

            }
            if (thisMP.west) {

            }
            if (thisMP.northEast) {

            }
            if (thisMP.northWest) {

            }
            if (thisMP.southEast) {

            }
            if (thisMP.southWest) {

            }


        }
        return returnSet;
    }
}
