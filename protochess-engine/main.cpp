#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    protochess_engine::Chess chess = protochess_engine::Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn("A2", "A3", 0) << std::endl;
    //std::cout << chess.takeTurn(3, 1, 3, 2,0);
    std::cout << chess.toString() << std::endl;
    return 0;
}
