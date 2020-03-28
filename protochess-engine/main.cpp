#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    protochess_engine::Chess chess = protochess_engine::Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("C2", "C4", 0).successful << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << "Printing checked player size:\n";
    std::cout << chess.takeTurn("E7", "E5", 1).checkmatedPlayers.size() << std::endl;
    std::cout << "end\n";
    std::cout << chess.toString() << std::endl;
    return 0;
}

