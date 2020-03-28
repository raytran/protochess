#include <iostream>

#include "shared/chess.h"
#include "bitsetutil.h"
#include "piecerules.h"
#include "rankfile.h"
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>

namespace protochess_engine {
    using boost::dynamic_bitset;
    std::string Chess::toString() {
        std::string returnString;
        returnString += gameState.toString();
        return returnString;
    }

    int Chess::registerPlayer(std::string playerName) {
        return gameState.registerPlayer(playerName);
    }

    void Chess::buildClassicSet() {
        reset();
        gameState = GameState({8, 8});

        gameState.registerPlayer("White"); //player num 0
        gameState.registerPlayer("Black"); //player num 1

        std::vector<char> wPieces = {
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', 'R', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                'P', 'P', 'P', 'P', 'P', 'P', 'P', ' ',
                'R', ' ', ' ', ' ', 'K', ' ', ' ', 'R'
        };

        std::vector<char> bPieces = {
                'r', ' ', ' ', ' ', 'k', 'b', 'n', 'r',
                'p', 'p', ' ', 'p', 'p', 'p', 'p', 'p',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '
        };
        gameState.setPieces(0, charToPieces(0, wPieces));
        gameState.setMovementMap(0, charToKnownMP(piecerules::moveRules, wPieces));
        gameState.setCaptureMap(0, charToKnownMP(piecerules::captureRules, wPieces));

        gameState.setPieces(1, charToPieces(1, bPieces));
        gameState.setMovementMap(1, charToKnownMP(piecerules::moveRules, bPieces));
        gameState.setCaptureMap(1, charToKnownMP(piecerules::captureRules, bPieces));
    }

    std::map<boost::uuids::uuid, std::shared_ptr<Piece>> Chess::charToPieces(int owner, std::vector<char> &pieces) {
        std::map<boost::uuids::uuid, std::shared_ptr<Piece>> returnMap;
        boost::uuids::random_generator generator;

        if (pieces.size() != gameState.getWidth() * gameState.getHeight()) {
            throw std::runtime_error("ERROR: invalid Piece array size");
        }

        int index = 0;
        for (int y = gameState.getHeight() - 1; y >= 0; y--) {
            for (int x = 0; x < gameState.getWidth(); x++) {
                if (pieces[index] != ' ') {
                    char charHere = pieces[index];
                    boost::uuids::uuid id = generator();
                    returnMap.insert(
                            std::make_pair(
                                    id,
                                    std::make_shared<Piece>(
                                            Piece(
                                                    charHere == 'k' || charHere == 'K',
                                                    owner,
                                                    id,
                                                    dynamic_bitset<>(gameState.getWidth()
                                                                     * gameState.getHeight()),
                                                    charHere,
                                                    {x, y},
                                                    bitsetUtil::getIndex(gameState.getWidth(),
                                                                         {x, y}
                                                    )
                                            )
                                    )
                            )
                    );

                    int i = bitsetUtil::getIndex(gameState.getWidth(), {x, y});
                    returnMap.at(id)->setLocation({x, y}, i);
                }
                index++;
            }
        }

        return returnMap;
    }

    void Chess::reset() {
        //Replace the gamestate
        gameState = GameState({8, 8});
    }

    std::map<char, MovementPattern>
    Chess::charToKnownMP(std::map<char, MovementPattern> &dictionary, std::vector<char> &pieces) {
        std::map<char, MovementPattern> returnMap;
        for (auto &x:pieces) {
            if (x != ' ') {
                if (dictionary.count(x) != 0) {
                    returnMap.insert(
                            std::make_pair(x, dictionary.at(x))
                    );
                }
            }
        }
        return returnMap;
    }

    bool Chess::takeTurn(int startX, int startY, int endX, int endY, int whosTurn) {
        Location start = {startX, startY};
        std::shared_ptr<Piece> startPiece = gameState.pieceAt(start);
        if (startPiece != nullptr && whosTurn == gameState.getWhosTurn()) {
            std::cout << "\n----";
            std::cout << "Start piece: \n";
            std::cout << startPiece;
            std::cout << "\n";
            boost::uuids::uuid idHere = startPiece->getId();

            Location end = {endX, endY};
            LocationDelta delta = {start, end};

            //Generate possible moves for this player
            //And check if the location delta matches
            std::map<boost::uuids::uuid, std::unordered_set<Move>> moves = gameState.generateMoves(whosTurn);
            if (moves.count(idHere) != 0) {
                for (auto &x : moves.at(idHere)) {
                    if (x.locationDelta == delta) {

                        std::cout << "After eval piece\n";
                        std::cout << gameState.pieceAt(delta.start);
                        std::cout << "\n-----\n";
                        //Viable move!
                        //Perform move
                        gameState.makeMove(x);
                        gameState.incrementTurn();
                        return true;
                    }
                }
            }
        }
        return false;
    }

    Chess::Chess() : gameState({8, 8}) {
    }

    bool Chess::takeTurn(const std::string &start, const std::string &end, int whosTurn) {
        return takeTurn(
                rankfile::toLocation(start).x,
                rankfile::toLocation(start).y,
                rankfile::toLocation(end).x,
                rankfile::toLocation(end).y,
                whosTurn
        );
    }
}
