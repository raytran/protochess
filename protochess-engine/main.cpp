#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    protochess_engine::Chess chess = protochess_engine::Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("C7", "C8", 0) << std::endl;
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("E7", "E5", 1) << std::endl;
    std::cout << chess.toString() << std::endl;
    return 0;
}

