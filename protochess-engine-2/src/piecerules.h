//
// Created by raytran on 4/1/20.
//
#pragma once
#include <unordered_map>
#include "types.h"

namespace protochess_engine::piecerules {
    extern std::unordered_map<char, MovementPattern> moveRules;
    extern std::unordered_map<char, MovementPattern> captureRules;
}
