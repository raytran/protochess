//
// Created by raytran on 3/26/20.
//
#pragma once

#include "types.h"

namespace protochess_engine::rankfile {
    Location toLocation(std::string rankFile);

    std::string toRankFile(Location loc);

}
