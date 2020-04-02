//
// Created by raytran on 4/1/20.
//

#include "include/protochess_engine.h"

int main(){
    protochess_engine::ProtochessEngine engine;
    //engine.loadFEN("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    std::cout<<engine.perft(5);
    std::cout<<'\n';
    std::cout<<engine.toString();






}