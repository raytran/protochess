//
// Created by raytran on 3/31/20.
//
#include "include/protochess_engine.h"
#include <iostream>
int main(){
    protochess_engine::ProtochessEngine engine(8,8);
    std::cout<< engine.toString();
    engine.getMoves(0);

    return 0;
}

