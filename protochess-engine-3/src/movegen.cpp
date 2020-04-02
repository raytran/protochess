//
// Created by raytran on 4/1/20.
//

#include "movegen.h"
#include "types.h"
#include "attacktables.h"
#include "bitboardutil.h"
using boost::multiprecision::msb;
using boost::multiprecision::lsb;
using boost::multiprecision::bit_set;
using boost::multiprecision::bit_unset;

namespace protochess_engine::movegen{
    namespace{
        bitboard calculatePositiveAttack(const Direction &dir,
                                         const AttackTables *attackTables,
                                         bitboard positiveAttack,
                                         const bitboard &allPieces) {
            bitboard blockers = positiveAttack & allPieces;
            if (blockers) {
                //Piece is being blocked
                positiveAttack ^= attackTables->getRayAttack(dir, lsb(blockers));
            }
            return positiveAttack;
        }

        bitboard calculateNegativeAttack(const Direction &dir,
                                         const AttackTables *attackTables,
                                         bitboard negativeAttack,
                                         const bitboard &allPieces) {
            bitboard blockers = negativeAttack & allPieces;
            if (blockers) {
                //Piece is being blocked
                negativeAttack ^= attackTables->getRayAttack(dir, msb(blockers));
            }
            return negativeAttack;
        }


        bitboard generateValidSquares(const bitboard &singlePiece,
                                      const MovementPattern *thisMP,
                                      const AttackTables *attackTables,
                                      const bitboard &allPlayerPieces,
                                      const bitboard &thisPlayerPieces) {
            //Determine which movement pattern to use

            //Squares that the piece can move to, including captures
            bitboard validSquares = 0;
            int locIndex = boost::multiprecision::msb(singlePiece);
            //SLIDING MOVES:
            //POSITIVE ATTACKS
            if (thisMP->north) {
                validSquares |= calculatePositiveAttack(NORTH, attackTables,
                                                        attackTables->getRayAttack(NORTH, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->east) {
                validSquares |= calculatePositiveAttack(EAST, attackTables,
                                                        attackTables->getRayAttack(EAST, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->northEast) {
                validSquares |= calculatePositiveAttack(NORTHEAST, attackTables,
                                                        attackTables->getRayAttack(NORTHEAST, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->northWest) {
                validSquares |= calculatePositiveAttack(NORTHWEST, attackTables,
                                                        attackTables->getRayAttack(NORTHWEST, locIndex),
                                                        allPlayerPieces);
            }

            //NEGATIVE ATTACKS
            if (thisMP->south) {
                validSquares |= calculateNegativeAttack(SOUTH, attackTables,
                                                        attackTables->getRayAttack(SOUTH, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->west) {
                validSquares |= calculateNegativeAttack(WEST, attackTables,
                                                        attackTables->getRayAttack(WEST, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->southEast) {
                validSquares |= calculateNegativeAttack(SOUTHEAST, attackTables,
                                                        attackTables->getRayAttack(SOUTHEAST, locIndex),
                                                        allPlayerPieces);
            }
            if (thisMP->southWest) {
                validSquares |= calculateNegativeAttack(SOUTHWEST, attackTables,
                                                        attackTables->getRayAttack(SOUTHWEST, locIndex),
                                                        allPlayerPieces);
            }

            //NONSLIDING PIECES
            for (auto &m : thisMP->deltas) {
                validSquares |= bitboardutil::translate(m, singlePiece, attackTables);
            }

            //Filter out attacks/moves on your own pieces
            validSquares &= ~thisPlayerPieces;
            validSquares &= attackTables->getBoundaryMask();

            return validSquares;
        }

        void generatePseudoLegalMoves(std::vector<Move> &returnMoves,
                                      const ProtochessEngine *engine,
                                      int whosTurn,
                                      const AttackTables *tables,
                                      const Position *position){



            const bitboard &allPieces = position->getAllPieces();
            const bitboard &thisPlayerPieces = position->getPlayerAllPieces()[whosTurn];


            bitboard enemies = allPieces & ~thisPlayerPieces;


            for (const auto &x : position->getPlayerPiecesMap()[whosTurn]){
                //Char type
                const char &c = x.first; //char
                const MovementPattern *CP = engine->getCapturePattern(c);
                const MovementPattern *TP = engine->getTranslatePattern(c);
                //This bitboard contains many bits for each piece
                //Make a copy of it to modify
                bitboard bits = x.second;
                while (bits){
                    uint32_t fromIndex = msb(bits);
                    bitboard singlePiece = 0;
                    bit_set(singlePiece,fromIndex);


                    //Modify the movement pattern if this is a pawn
                    //that is not in the right position
                    if (c == 'p' || c == 'P'){
                        Location loc = bitboardutil::getLoc(position->getDimensions().width,fromIndex);
                        if (c=='P'){
                            if (loc.y != 1){
                                TP = engine->getTranslatePattern('.');
                            }else{
                                //Reset for initial pawns
                                TP = engine->getTranslatePattern(c);
                            }
                        }else{
                            if (loc.y != position->getDimensions().height - 2){
                                TP = engine->getTranslatePattern(',');
                            }else{
                                TP = engine->getTranslatePattern(c);
                            }
                        }
                    }

                    //TRANSLATES
                    bitboard validTranslates = generateValidSquares(singlePiece,TP,tables,allPieces,thisPlayerPieces);
                    validTranslates &= ~enemies;

                    while(validTranslates){
                        uint32_t toIndex = msb(validTranslates);
                        returnMoves.push_back(Move(fromIndex,toIndex,' ',false, {-1, ' '}));
                        bit_unset(validTranslates,toIndex);
                    }

                    //CAPTURES
                    bitboard validCaptures = generateValidSquares(singlePiece,CP,tables,allPieces,thisPlayerPieces);
                    validCaptures &= enemies;

                    while(validCaptures){
                        int toIndex = msb(validCaptures);
                        returnMoves.push_back(Move(fromIndex,toIndex,' ',true, position->pieceAt(toIndex)));
                        bit_unset(validCaptures,toIndex);
                    }

                    //Go to the next one
                    bit_unset(bits,fromIndex);
                }
            }
        }
    }

    std::vector<Move> generateLegalMoves(const ProtochessEngine *engine, const AttackTables *tables, int whosTurn, const Position *position) {
        std::vector<Move> returnMoves;
        generatePseudoLegalMoves(returnMoves,engine, whosTurn, tables, position);
        return returnMoves;
    }
};

