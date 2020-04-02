//
// Created by raytran on 4/1/20.
//

#include "movegen.h"
#include <boost/dynamic_bitset.hpp>
#include "types.h"
#include "bitsetutil.h"
#include "attacktables.h"
#include "gamestate.h"

using boost::dynamic_bitset;
using std::map;


namespace protochess_engine {

    namespace{
        std::vector<LocationDelta> bitboardsToDeltas(const Dimensions &dimensions,
                                                     const dynamic_bitset<> &onePiece,
                                                     dynamic_bitset<> destinations) {

            std::vector<LocationDelta> returnSet;
            Location start = bitsetUtil::getLoc(dimensions.width, onePiece.find_first());

            while (destinations.find_first() != dynamic_bitset<>::npos) {
                size_t index = destinations.find_first();
                Location end = bitsetUtil::getLoc(dimensions.width, index);
                returnSet.push_back({start, end});
                destinations.set(index, false);
            }
            return returnSet;
        }

        dynamic_bitset<> calculatePositiveAttacks(const Direction &dir,
                                                  const AttackTables &attackTables,
                                                  const dynamic_bitset<> &positiveAttack,
                                                  const dynamic_bitset<> &allPieces) {
            dynamic_bitset<> returnAttack(positiveAttack);
            dynamic_bitset<> blockers = positiveAttack & allPieces;
            if (blockers.any()) {
                //Piece is being blocked
                returnAttack = positiveAttack ^ attackTables.getRayAttack(dir, blockers.find_first());
            }
            return returnAttack;
        }

        dynamic_bitset<> calculateNegativeAttacks(const Direction &dir,
                                                  const AttackTables &attackTables,
                                                  const dynamic_bitset<> &negativeAttack,
                                                  const dynamic_bitset<> &allPieces) {
            dynamic_bitset<> returnAttack(negativeAttack);
            dynamic_bitset<> blockers = negativeAttack & allPieces;
            if (blockers.any()) {
                //Piece is being blocked
                returnAttack = negativeAttack ^ attackTables.getRayAttack(dir, bitsetUtil::findLast(blockers));
            }
            return returnAttack;
        }

        //Given a bitset for a single piece and a movement pattern, returns the list of ALL valid squares
        dynamic_bitset<> generateValidSquares(const dynamic_bitset<> &thisPiece,
                                              char charRep,
                                              const MovementPattern &thisMP,
                                              const AttackTables &attackTables,
                                              const dynamic_bitset<> &allPlayerPieces,
                                              const dynamic_bitset<> &thisPlayerPieces,
                                              const unordered_map<char, dynamic_bitset<>> &initialPlayerPieces) {
            //Determine which movement pattern to use

            //Squares that the piece can move to, including captures
            dynamic_bitset<> validSquares(allPlayerPieces.size());
            int locIndex = thisPiece.find_first();
            //SLIDING MOVES:
            //POSITIVE ATTACKS
            if (thisMP.north) {
                validSquares |= calculatePositiveAttacks(NORTH, attackTables,
                                                         attackTables.getRayAttack(NORTH, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.east) {
                validSquares |= calculatePositiveAttacks(EAST, attackTables,
                                                         attackTables.getRayAttack(EAST, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.northEast) {
                validSquares |= calculatePositiveAttacks(NORTHEAST, attackTables,
                                                         attackTables.getRayAttack(NORTHEAST, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.northWest) {
                validSquares |= calculatePositiveAttacks(NORTHWEST, attackTables,
                                                         attackTables.getRayAttack(NORTHWEST, locIndex),
                                                         allPlayerPieces);
            }

            //NEGATIVE ATTACKS
            if (thisMP.south) {
                validSquares |= calculateNegativeAttacks(SOUTH, attackTables,
                                                         attackTables.getRayAttack(SOUTH, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.west) {
                validSquares |= calculateNegativeAttacks(WEST, attackTables,
                                                         attackTables.getRayAttack(WEST, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.southEast) {
                validSquares |= calculateNegativeAttacks(SOUTHEAST, attackTables,
                                                         attackTables.getRayAttack(SOUTHEAST, locIndex),
                                                         allPlayerPieces);
            }
            if (thisMP.southWest) {
                validSquares |= calculateNegativeAttacks(SOUTHWEST, attackTables,
                                                         attackTables.getRayAttack(SOUTHWEST, locIndex),
                                                         allPlayerPieces);
            }

            //NONSLIDING PIECES
            for (auto &m : thisMP.deltas) {
                //Edge case pawn movement
                if ((charRep == 'p' || charRep == 'P')
                    && !(initialPlayerPieces.at('p') & thisPiece).any()) {
                    if (m.y == 2 || m.y == -2) {
                        continue;
                    }
                }
                validSquares |= bitsetUtil::translate(m, thisPiece, attackTables);
            }

            //Filter out attacks/moves on your own pieces
            validSquares &= ~thisPlayerPieces;

            return validSquares;
        }
    }


    std::unordered_set<Move> MoveGenerator::generatePseudoLegalMoves(int whosTurn) const{
        std::unordered_set<Move> returnMoves;
        const std::unordered_map<char, dynamic_bitset<>> &thisPlayer = gameState.playerPieces[whosTurn];
        const std::unordered_map<char, dynamic_bitset<>> &thisPlayerInitialPieces = gameState.initialPlayerPieces[whosTurn];
        dynamic_bitset<> enemies(gameState.dimensions.width * gameState.dimensions.height);

        enemies = gameState.allPieces & ~gameState.playerPiecesAll[whosTurn];


        //For each piecetype of this player
        for (auto &x:thisPlayer){
            MovementPattern &thisMP = gameState.moveRules.at('k');
            MovementPattern &thisCP = gameState.captureRules.at('k');
            if (gameState.moveRules.contains(x.first)){
                thisMP = gameState.moveRules.at(x.first);
            }else{
                std::cerr << "WARNING: no move rule found for ";
                std::cerr << x.first;
                std::cerr << ". Defaulting to king-style movement";
            }

            if (gameState.captureRules.contains(x.first)){
                thisCP = gameState.captureRules.at(x.first);
            }else{
                std::cerr << "WARNING: no capture rule found for ";
                std::cerr << x.first;
                std::cerr << ". Defaulting to king-style movement";
            }


            //Extract this piece
            dynamic_bitset<> piecesCopy = x.second;
            while (piecesCopy.any()){
                size_t index = piecesCopy.find_first();
                dynamic_bitset<> current(gameState.dimensions.width * gameState.dimensions.height);
                //Get only this piece
                current.set(index,true);



                dynamic_bitset<> validMovements = generateValidSquares(current,
                                                                       x.first,
                                                                       thisMP,
                                                                       gameState.attackTables,
                                                                       gameState.allPieces,
                                                                       gameState.playerPiecesAll[whosTurn],
                                                                       thisPlayerInitialPieces);

                //Filter out attacks when looking at movements
                validMovements &= ~enemies;


                if (validMovements.any()){
                    std::vector<LocationDelta> deltas = bitboardsToDeltas(gameState.dimensions,
                                                                          current,
                                                                          validMovements);
                    for (auto &z:deltas){
                        std::cout<<"INSERTING MOVE\n";
                        returnMoves.insert({
                                                   false,
                                                   ' ',
                                                   false,
                                                   false,
                                                   false,
                                                   z.start,
                                                   z.end
                                           });
                    }
                }


                dynamic_bitset<> validCaptures = generateValidSquares(current,
                                                                      x.first,
                                                                      thisCP,
                                                                      gameState.attackTables,
                                                                      gameState.allPieces,
                                                                      gameState.playerPiecesAll[whosTurn],
                                                                      thisPlayerInitialPieces);
                validCaptures &= enemies;

                if (validCaptures.any()){
                    std::vector<LocationDelta> deltas = bitboardsToDeltas(gameState.dimensions,
                                                                          current,
                                                                          validCaptures);
                    for (auto &z:deltas){
                        returnMoves.insert({
                                                   false,
                                                   ' ',
                                                   true,
                                                   false,
                                                   false,
                                                   z.start,
                                                   z.end
                                           });
                    }
                }

                //Move to the next one by clearing this bit
                piecesCopy.reset(index);
            }
        }
        return returnMoves;
    }

    MoveGenerator::MoveGenerator(GameState &gameState):gameState(gameState){
    }
};
