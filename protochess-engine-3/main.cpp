//
// Created by raytran on 4/1/20.
//

#include "include/protochess_engine.h"

int main(){
    protochess_engine::ProtochessEngine engine;
    std::cout << engine.perft(3);
    std::cout << '\n';
    std::cout<<engine.toString();






}