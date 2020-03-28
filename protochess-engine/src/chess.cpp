#include <iostream>

#include "shared/chess.h"
#include "bitsetutil.h"
#include "piecerules.h"
#include "rankfile.h"
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <chrono>

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
                ' ', ' ', 'P', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                'P', 'P', 'P', 'P', 'P', 'P', 'P', ' ',
                'R', ' ', ' ', 'Q', 'K', ' ', ' ', 'R'
        };

        std::vector<char> bPieces = {
                ' ', ' ', ' ', ' ', 'k', 'b', 'n', 'r',
                'p', ' ', ' ', 'p', 'p', 'p', 'p', 'p',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '
        };
        gameState.setPieces(0, charToPieces(0, wPieces));
        gameState.setMovementMap(0, piecerules::moveRules);
        gameState.setCaptureMap(0, piecerules::captureRules);

        gameState.setPieces(1, charToPieces(1, bPieces));
        gameState.setMovementMap(1, piecerules::moveRules);
        gameState.setCaptureMap(1, piecerules::captureRules);
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

                    bool promotable = charHere == 'p' || charHere == 'P';
                    char promoteTo = ' ';
                    if (promotable) {
                        if (charHere == 'p') promoteTo = 'q';
                        else promoteTo = 'Q';
                    }
                    returnMap.insert(
                            std::make_pair(
                                    id,
                                    std::make_shared<Piece>(
                                            Piece(
                                                    promotable,
                                                    promoteTo,
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

    TurnResult Chess::takeTurn(int startX, int startY, int endX, int endY, int whosTurn) {
        Location start = {startX, startY};
        std::shared_ptr<Piece> startPiece = gameState.pieceAt(start);
        if (startPiece != nullptr && whosTurn == gameState.getWhosTurn()) {
            boost::uuids::uuid idHere = startPiece->getId();

            Location end = {endX, endY};
            LocationDelta delta = {start, end};

            auto start = std::chrono::high_resolution_clock::now();
            std::map<boost::uuids::uuid, std::unordered_set<Move>> moves = gameState.generateMoves(whosTurn);

            auto elapsed = std::chrono::high_resolution_clock::now() - start;

            long long microseconds = std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count();

            std::cout << "\nLEGAL MOVES TOOK:";
            std::cout << microseconds;
            std::cout << " MICROSECONDS\n";


            //Generate possible moves for this player
            //And check if the location delta matches
            if (moves.count(idHere) != 0) {
                for (auto &x : moves.at(idHere)) {
                    if (x.locationDelta == delta) {

                        //Viable move!
                        //Perform move
                        gameState.makeMove(x);
                        gameState.incrementTurn();
                        return {true,
                                gameState.getWhosTurn(),
                                gameState.getCheckedPlayers(),
                                gameState.getCheckMatedPlayers()};
                    }
                }
            }
        }

        return {false,
                gameState.getWhosTurn(),
                gameState.getCheckedPlayers(),
                gameState.getCheckMatedPlayers()};
    }

    Chess::Chess() : gameState({8, 8}) {
    }

    TurnResult Chess::takeTurn(const std::string &start, const std::string &end, int whosTurn) {
        return takeTurn(
                rankfile::toLocation(start).x,
                rankfile::toLocation(start).y,
                rankfile::toLocation(end).x,
                rankfile::toLocation(end).y,
                whosTurn
        );
    }
}
