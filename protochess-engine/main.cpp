#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    Chess chess = Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn(3, 1, 3, 2);
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn(1, 7, 2, 5);
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn(3, 2, 3, 3);
    std::cout << chess.toString() << std::endl;
    std::cout << chess.takeTurn(2, 5, 3, 3);
    std::cout << chess.toString() << std::endl;
    return 0;
}
