//
// Created by raytran on 4/1/20.
//

#include "../include/protochess_engine.h"
#include "bitboardutil.h"
#include "piecerules.h"
#include "movegen.h"
#include "rankfile.h"

using boost::multiprecision::bit_set;
using std::vector;
using std::unordered_map;
namespace protochess_engine{
    std::string ProtochessEngine::toString() {
        if (currentPosition != nullptr){
            return currentPosition->toString();
        }else return "No current position.\n";
    }

    //TODO finish other FEN aspects
    void ProtochessEngine::loadFEN(std::string fenString) {
        //Set width and height to 8x8
        dimensions.width = 8;
        dimensions.height = 8;
        attackTables.reset();
        attackTables = std::make_unique<AttackTables>(dimensions);


        vector<unordered_map<char, bitboard>> startPos;
        startPos.push_back(unordered_map<char,bitboard>()); //0 for white
        startPos.push_back(unordered_map<char,bitboard>()); //1 for black

        int field = 0;
        int x = 0;
        int y = 7;
        for (auto &c:fenString){
            if (c == ' ') field++;
            switch(field){
                case 0:
                    if (c == '/'){
                        x=0;
                        y--;
                    }
                    else if (isdigit(c)) x += (int) c - 48;
                    else{
                        bitboard current = 0;
                        unordered_map<char,bitboard> &maphere = startPos[isupper(c) ? 0 : 1];
                        if (maphere.contains(c)){
                            current = maphere.at(c);
                        }
                        bit_set(current,bitboardutil::getIndex(8,x,y));
                        maphere.insert_or_assign(c,current);

                        x++;
                    }

                    break;
            }
        }

        currentPosition.reset();
        currentPosition = std::make_unique<Position>(Position({8,8},startPos));

    }

    //Loads defaults
    ProtochessEngine::ProtochessEngine(): dimensions({8,8}) {
        loadFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    const MovementPattern *ProtochessEngine::getCapturePattern(char c) const{
        //Check internal rules
        if (piecerules::captureRules.contains(c)){
            return &piecerules::captureRules.at(c);
        }else{
            //Eventually check player-defined rules
            std::cerr << "WARNING: no capture rule defined; using king-style";
            return &piecerules::captureRules.at('k');
        }
    }

    const MovementPattern *ProtochessEngine::getTranslatePattern(char c) const{
        //Check internal rules
        if (piecerules::moveRules.contains(c)){
            return &piecerules::moveRules.at(c);
        }else{
            std::cerr << "WARNING: no capture rule defined; using king-style";
            return &piecerules::moveRules.at('k');
        }
    }




    unsigned long long ProtochessEngine::perft_(int depth, int whosTurn = 0) {
        unsigned long long nodes = 0;

        std::vector<Move> moves = movegen::generateLegalMoves(this,
                                                              attackTables.get(),
                                                              whosTurn,
                                                              currentPosition.get());
        if (depth == 0) return 1;
        for (auto &move : moves) {
            currentPosition->makeMove(move);
            nodes += perft_(depth - 1, (whosTurn + 1) % numPlayers);
            currentPosition->unmakeMove(move);
        }

        return nodes;
    }

    unsigned long long ProtochessEngine::perft_fast_(int depth, int whosTurn = 0) {
        unsigned long long nodes = 0;

        std::vector<Move> moves = movegen::generateLegalMoves(this,
                                                              attackTables.get(),
                                                              whosTurn,
                                                              currentPosition.get());
        if (depth == 1) return moves.size();
        for (auto &move : moves) {
            currentPosition->makeMove(move);
            nodes += perft_(depth - 1, (whosTurn + 1) % numPlayers);
            currentPosition->unmakeMove(move);
        }
        return nodes;
    }

    unsigned long long ProtochessEngine::perft_divide_(int depth, int whosTurn) {

        unsigned long long nodes = 0;

        std::vector<Move> moves = movegen::generateLegalMoves(this,
                                                              attackTables.get(),
                                                              whosTurn,
                                                              currentPosition.get());
        for (auto &move : moves) {
            currentPosition->makeMove(move);
            unsigned long long additional = perft_(depth - 1, (whosTurn + 1) % numPlayers);

            Location from = bitboardutil::getLoc(8, move.getFrom());
            Location to = bitboardutil::getLoc(8, move.getTo());
            std::cout << currentPosition->pieceAt(from.x, from.y).charType;
            std::cout << rankfile::toRankFile(from);
            std::cout << "-";
            std::cout << rankfile::toRankFile(to);
            std::cout << " :";
            std::cout << additional;
            std::cout << "\n";
            nodes += additional;
            currentPosition->unmakeMove(move);

        }
        return nodes;
    }


    unsigned long long ProtochessEngine::perft(int depth) {
        return perft_fast_(depth, 1);
    }

    unsigned long long ProtochessEngine::perft_divide(int depth) {
        return perft_divide_(depth, 1);
    }
}
