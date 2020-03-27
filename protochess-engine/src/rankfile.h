//
// Created by raytran on 3/26/20.
//
#pragma once

#include "types.h"

namespace protochess_engine {
    namespace rankfile {
        Location toLocation(std::string rankFile) {
            char file = rankFile[0];
            rankFile.erase(0, 1);
            int rank = std::stoi(rankFile);
            return {file - 65, rank - 1};
        }
    }
}
