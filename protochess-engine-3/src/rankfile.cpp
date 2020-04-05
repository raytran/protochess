//
// Created by raytran on 4/5/20.
//

#include "rankfile.h"
#include <string>
#include "types.h"

namespace protochess_engine::rankfile {
    Location toLocation(std::string rankFile) {
        char file = rankFile[0];
        rankFile.erase(0, 1);
        int rank = std::stoi(rankFile);
        return {file - 65, rank - 1};
    }

    std::string toRankFile(Location loc) {
        std::string returnString;
        returnString += (char) (loc.x + 65);
        returnString += std::to_string(loc.y + 1);
        return returnString;
    }
}

