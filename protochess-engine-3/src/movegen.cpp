//
// Created by raytran on 4/1/20.
//

#include "movegen.h"
#include "types.h"
#include "attacktables.h"
#include "bitboardutil.h"
#include "bitboard.h"

namespace protochess_engine::movegen {
    namespace {
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
            bitboard validSquares;
            int locIndex = lsb(singlePiece);
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

            //Handle Jumps
            for (auto &m : thisMP->deltas) {
                int width = attackTables->getWidth();
                Location plusDelta = bitboardutil::getLoc(width, locIndex);
                plusDelta.x += m.x;
                plusDelta.y += m.y;
                if (plusDelta.x >= 0 && plusDelta.y >= 0 && plusDelta.x < width &&
                    plusDelta.y < attackTables->getHeight()) {
                    bit_set(validSquares, bitboardutil::getIndex(width, plusDelta.x, plusDelta.y));
                }
            }
            //Handle sliding in dx/dy style
            for (auto &m : thisMP->slideDeltas) {
                int width = attackTables->getWidth();
                Location plusDelta = bitboardutil::getLoc(width, locIndex);
                plusDelta.x += m.x;
                plusDelta.y += m.y;
                if (plusDelta.x >= 0 && plusDelta.y >= 0 && plusDelta.x < width &&
                    plusDelta.y < attackTables->getHeight()) {
                    //Apply the transform; stop at the first blocker
                    int index = bitboardutil::getIndex(width, plusDelta.x, plusDelta.y);
                    bit_set(validSquares, index);
                    //If this intersects another piece, we leave the loop
                    if (bit_test(allPlayerPieces, index)) break;
                } else {
                    //Break if we leave the board
                    break;
                }
            }

            //Filter out attacks/moves on your own pieces
            validSquares &= ~thisPlayerPieces;
            //validSquares &= attackTables->getBoundaryMask();

            return validSquares;
        }

        void generatePseudoLegalMoves(std::vector<Move> &returnMoves,
                                      const ProtochessEngine *engine,
                                      int whosTurn,
                                      const AttackTables *tables,
                                      const Rulebook *rulebook,
                                      const Position *position) {


            const bitboard &allPieces = position->getAllPieces();
            const bitboard &thisPlayerPieces = position->getPlayerAllPieces()[whosTurn];


            bitboard enemies = allPieces & ~thisPlayerPieces;


            for (const auto &x : position->getPlayerPiecesMap()[whosTurn]) {
                //Char type
                const char &c = x.first; //char
                const MovementPattern *CP = rulebook->getCapturePattern(c);
                const MovementPattern *TP = rulebook->getTranslatePattern(c);
                //This bitboard contains many bits for each piece
                //Make a copy of it to modify
                bitboard bits = x.second;
                while (bits) {
                    uint32_t fromIndex = lsb(bits);
                    bitboard singlePiece;
                    bit_set(singlePiece, fromIndex);


                    //Modify the movement pattern if this is a pawn
                    //that is not in the right position
                    if (c == 'p' || c == 'P') {
                        Location loc = bitboardutil::getLoc(position->getDimensions().width, fromIndex);
                        if (c == 'P') {
                            if (loc.y != 1) {
                                TP = rulebook->getTranslatePattern('.');
                            } else {
                                //Reset for initial pawns
                                TP = rulebook->getTranslatePattern(c);
                            }
                        } else {
                            if (loc.y != position->getDimensions().height - 2) {
                                TP = rulebook->getTranslatePattern(',');
                            } else {
                                TP = rulebook->getTranslatePattern(c);
                            }
                        }
                    }

                    //TRANSLATES
                    bitboard validTranslates = generateValidSquares(singlePiece, TP, tables, allPieces,
                                                                    thisPlayerPieces);
                    validTranslates &= ~enemies;

                    while (validTranslates) {
                        uint32_t toIndex = lsb(validTranslates);
                        returnMoves.push_back(Move(fromIndex, toIndex, ' ', false, {-1, ' '}));
                        bit_unset(validTranslates, toIndex);
                    }

                    //CAPTURES
                    bitboard validCaptures = generateValidSquares(singlePiece, CP, tables, allPieces, thisPlayerPieces);
                    validCaptures &= enemies;

                    while (validCaptures) {
                        int toIndex = lsb(validCaptures);
                        returnMoves.push_back(Move(fromIndex, toIndex, ' ', true, position->pieceAt(toIndex)));
                        bit_unset(validCaptures, toIndex);
                    }

                    //Go to the next one
                    bit_unset(bits, fromIndex);
                }
            }
        }

        //Returns the set of squares for a playerNum that is attacked by other players
        bitboard getDangerMap(int playerNum,
                              const ProtochessEngine *engine,
                              const AttackTables *tables,
                              const Rulebook *rulebook,
                              const Position *position) {

            bitboard dangerSquares;
            const bitboard &allPieces = position->getAllPieces();
            //CAPTURES
            for (int i = 0; i < position->getPlayerPiecesMap().size(); i++) {
                if (i == playerNum) continue;
                //Recalculate thisPlayerPieces (relative to this player)
                const bitboard &thisPlayerPieces = position->getPlayerAllPieces()[i];
                for (const auto &x : position->getPlayerPiecesMap()[i]) {
                    const MovementPattern *CP = rulebook->getCapturePattern(x.first);
                    //This bitboard contains many bits for each piece
                    //Make a copy of it to modify
                    bitboard bits = x.second;
                    while (bits) {
                        uint32_t fromIndex = lsb(bits);
                        bitboard singlePiece;
                        bit_set(singlePiece, fromIndex);

                        //CAPTURES
                        bitboard validCaptures = generateValidSquares(singlePiece,
                                                                      CP,
                                                                      tables,
                                                                      allPieces,
                                                                      thisPlayerPieces);
                        dangerSquares |= validCaptures;

                        //Go to the next one
                        bit_unset(bits, fromIndex);
                    }
                }
            }
            return dangerSquares;
        }

        //Given: a well formed pseudo move
        //Returns: whether or not this move would put the player in check
        bool isMoveLegal(int whosTurn, Move &move, const ProtochessEngine *engine, const AttackTables *tables,
                         const Rulebook *rulebook,
                         Position *position) {
            //Position copy = *position;
            position->makeMove(move);
            //Find the king
            int kingIndex = 0;
            for (const auto &x : position->getPlayerPiecesMap()[whosTurn]) {
                if (x.first == 'k' || x.first == 'K') {
                    kingIndex = lsb(x.second);
                    break;
                }
            }
            //Calculate the dangerMap
            bitboard dangerMap = getDangerMap(whosTurn, engine, tables, rulebook, position);
            position->unmakeMove(move);

            //assert(copy == *position);
            return (bit_test(dangerMap, kingIndex)) == 0;
        }
    }

    std::vector<Move>
    generateLegalMoves(const ProtochessEngine *engine, const AttackTables *tables, const Rulebook *rulebook,
                       int whosTurn, Position *position) {
        std::vector<Move> pseudoMoves;
        generatePseudoLegalMoves(pseudoMoves, engine, whosTurn, tables, rulebook, position);

        std::vector<Move> returnMoves;
        for (int i = 0; i < pseudoMoves.size(); i++) {
            if (isMoveLegal(whosTurn, pseudoMoves[i], engine, tables, rulebook, position)) {

                returnMoves.push_back(pseudoMoves[i]);
            }
        }

        return returnMoves;
    }
};

