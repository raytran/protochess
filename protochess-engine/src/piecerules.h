//
// Created by raytran on 3/25/20.
//

#pragma once


#include <map>
#include "types.h"

namespace protochess_engine {
    namespace piecerules {
        extern std::map<char, MovementPattern> moveRules;
        extern std::map<char, MovementPattern> captureRules;
    }
}

