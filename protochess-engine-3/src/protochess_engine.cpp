//
// Created by raytran on 4/1/20.
//

#include "../include/protochess_engine.h"
#include "bitboardutil.h"
#include "piecerules.h"
#include "movegen.h"
#include "rankfile.h"
#include "bitboard.h"
#include "perft.h"

using boost::multiprecision::bit_set;
using std::vector;
using std::unordered_map;
namespace protochess_engine {
    std::string ProtochessEngine::toString() {
        if (currentPosition != nullptr) {
            return currentPosition->toString();
        } else return "No current position.\n";
    }

    //TODO finish other FEN aspects (who's moving, castle etc)
    void ProtochessEngine::loadFEN(std::string fenString) {
        //Set width and height to 8x8
        dimensions.width = 8;
        dimensions.height = 8;
        attackTables.reset();
        attackTables = std::make_unique<AttackTables>(dimensions);

        pieceRulebook.reset();
        pieceRulebook = std::make_unique<Rulebook>();

        vector<unordered_map<char, bitboard>> startPos;
        startPos.emplace_back(); //0 for white
        startPos.emplace_back(); //1 for black

        int field = 0;
        int x = 0;
        int y = 7;
        for (auto &c:fenString) {
            if (c == ' ') field++;
            switch (field) {
                case 0:
                    if (c == '/') {
                        x = 0;
                        y--;
                    } else if (isdigit(c)) x += (int) c - 48;
                    else {
                        bitboard current;
                        unordered_map<char, bitboard> &maphere = startPos[isupper(c) ? 0 : 1];
                        if (maphere.contains(c)) {
                            current = maphere.at(c);
                        }
                        bit_set(current, bitboardutil::getIndex(8, x, y));
                        maphere.insert_or_assign(c, current);

                        x++;
                    }

                    break;
            }
        }

        currentPosition.reset();
        currentPosition = std::make_unique<Position>(Position({8, 8}, startPos));

    }

    //Loads defaults
    ProtochessEngine::ProtochessEngine() : dimensions({8, 8}) {
        loadFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }


    unsigned long long ProtochessEngine::perft(int depth) {
        return perft_tests::perft(depth,
                                  this,
                                  attackTables.get(),
                                  pieceRulebook.get(),
                                  currentPosition.get());
    }

    unsigned long long ProtochessEngine::perft_divide(int depth) {
        return perft_tests::perft_divide(depth,
                                         this,
                                         attackTables.get(),
                                         pieceRulebook.get(),
                                         currentPosition.get());
    }

    bool ProtochessEngine::makeMove(std::string from, std::string to) {
        Location fromLoc = rankfile::toLocation(from);
        Location toLoc = rankfile::toLocation(to);

        std::vector<Move> moves = movegen::generateLegalMoves(this,
                                                              attackTables.get(),
                                                              pieceRulebook.get(),
                                                              whosTurn,
                                                              currentPosition.get());

        for (const auto &m:moves) {
            if (m.getFrom() == bitboardutil::getIndex(dimensions.width, fromLoc.x, fromLoc.y)
                && m.getTo() == bitboardutil::getIndex(dimensions.width, toLoc.x, toLoc.y)) {
                if (currentPosition->pieceAt(m.getFrom()).owner == whosTurn) {
                    currentPosition->makeMove(m);
                    whosTurn = (whosTurn + 1) % numPlayers;
                    return true;
                } else return false;
            }
        }

        return false;
    }

    int ProtochessEngine::getWhosTurn() {
        return whosTurn;
    }

    int ProtochessEngine::getNumPlayers() {
        return numPlayers;
    }

}
