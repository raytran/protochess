//
// Created by raytran on 4/1/20.
//


#include <chrono>
#include "include/protochess_engine.h"
#include "src/types.h"

int main() {

    protochess_engine::ProtochessEngine engine;
    std::cout << engine.perft_divide(1);
    std::cout << engine.toString();

}