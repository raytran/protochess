#include <iostream>
#include "shared/chess.h"

int main(int argc, char *argv[]) {
    Chess chess = Chess();
    chess.buildClassicSet();
    std::cout << chess.toString() << std::endl;
    return 0;
}
