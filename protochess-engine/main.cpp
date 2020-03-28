#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    protochess_engine::Chess chess = protochess_engine::Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("E1", "C1", 0) << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("E7", "E5", 1) << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("E2", "E4", 0) << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("E8", "C8", 1) << std::endl;
    std::cout << chess.toString() << std::endl;
    return 0;
}

