#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    protochess_engine::Chess chess = protochess_engine::Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("A2", "A3", 0) << std::endl;
    //std::cout << chess.takeTurn(3, 1, 3, 2,0);
    std::cout << chess.takeTurn("D4", "A4", 1) << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("A3", "B3", 0) << std::endl;
    std::cout << chess.toString() << std::endl;
    return 0;
}

/*
            std::map<boost::uuids::uuid, std::unordered_set<Move>>
            generateCastlingPseudoMoves_(GameState &gameState,
                                         std::map<boost::uuids::uuid, std::unordered_set<Move>> &pseudoMoves,
                                         Player &player,
                                         Board &board){
                std::map<boost::uuids::uuid, std::unordered_set<Move>> returnSet = {};

                if (player.getCastleRights().castleAble){
                    boost::dynamic_bitset<> allPlayerPieces = boost::dynamic_bitset<>(board.getAllPieces());
                    //Generate all of this player's pieces
                    boost::dynamic_bitset<> thisPlayerPieces = player.getAllPiecesBitset();
                    //generate enemies
                    boost::dynamic_bitset<> enemyPieces;
                    enemyPieces = allPlayerPieces & (~thisPlayerPieces);


                    //Find the king

                    for (auto &x:player.getPieces()){
                        //Check if this piece is a king that hasn't moved before
                        if ((x.second->getCharRep() == 'k' || x.second->getCharRep() == 'K')
                            && !x.second->getMovedBefore()){

                            bool isWhite = x.second->getCharRep() == 'K';
                            //Check if this player can castle
                            //KINGSIDE CASTLING
                            if (player.getCastleRights().kingSide){
                                //Check if player can move one square kingside
                                //Find the move to the right, if it exists
                                for (auto &z:pseudoMoves.at(x.first)){
                                    //Make sure this is not a capture move
                                    if (!z.capture
                                        && z.locationDelta.end.x - z.locationDelta.start.x == 1
                                        && z.locationDelta.end.y - z.locationDelta.start.y == 0){
                                        if (isMoveValidForPlayer(player.getPlayerNum(),gameState,board,z)){
                                            //Check if the next square is not occupied & if the leftmost square
                                            //Is occupied by a piece that is a rook
                                            Location end = z.locationDelta.end;
                                            end.x += 1;
                                            boost::dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
                                            nextSquare.set(bitsetUtil::getIndex(board.getWidth(), end), true);
                                            Location castlingRookLoc = end;
                                            castlingRookLoc.x += 1;
                                            std::shared_ptr<Piece> castlingRook = gameState.pieceAt(castlingRookLoc);
                                            if (!(nextSquare & allPlayerPieces).any()
                                                && castlingRook != nullptr
                                                && castlingRook->getOwner() == player.getPlayerNum()
                                                && (castlingRook->getCharRep() == 'r' || castlingRook->getCharRep() == 'R')){
                                                //Add the pseudo move
                                                if (returnSet.count(x.first) == 0) {
                                                    returnSet.insert({x.first, std::unordered_set<Move>()});
                                                }
                                                returnSet.at(x.first).insert({false,
                                                                              nullptr,
                                                                              true,
                                                                              false,
                                                                              {z.locationDelta.start,end}
                                                                             });
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                            //QUEENSIDE CASTLING
                            if (player.getCastleRights().queenSide){
                                //Check if player can move one square queenside
                                //Find the move to the left, if it exists
                                for (auto &z:pseudoMoves.at(x.first)){
                                    //Make sure this is not a capture move
                                    if (!z.capture
                                        && z.locationDelta.end.x - z.locationDelta.start.x == -1
                                        && z.locationDelta.end.y - z.locationDelta.start.y == 0){
                                        if (isMoveValidForPlayer(player.getPlayerNum(),gameState,board,z)){
                                            //Check if the next square is not occupied

                                            Location end = z.locationDelta.end;
                                            end.x -= 1;
                                            boost::dynamic_bitset<> nextSquare(board.getWidth() * board.getHeight());
                                            nextSquare.set(bitsetUtil::getIndex(board.getWidth(), end), true);
                                            if (!(nextSquare & allPlayerPieces).any()){
                                                //Add the pseudo move
                                                if (returnSet.count(x.first) == 0) {
                                                    returnSet.insert({x.first, std::unordered_set<Move>()});
                                                }
                                                returnSet.at(x.first).insert({false,
                                                                              nullptr,
                                                                              false,
                                                                              true,
                                                                              {z.locationDelta.start,end}
                                                                             });
                                            }
                                        }
                                        break;
                                    }
                                }
                            }

                            break; //king check
                        }
                    }
                }
                return returnSet;
            }
        }
*/
