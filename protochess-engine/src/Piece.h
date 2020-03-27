//
// Created by raytran on 3/25/20.
//

#pragma once

#include <boost/uuid/uuid.hpp>
#include <boost/dynamic_bitset.hpp>
#include "types.h"

namespace protochess_engine {
    class Piece {
    private:
        char charRep;
        int owner;
        bool movedBefore = false;
        bool appliesCheck;
        boost::uuids::uuid id;
        boost::dynamic_bitset<> bitset;
        Location location;
        int locationIndex;
    public:
        Piece(bool appliesCheck,
              int owner,
              boost::uuids::uuid id,
              boost::dynamic_bitset<> bitset,
              char charRep,
              Location loc,
              int locationIndex);

        bool getMovedBefore();

        bool getAppliesCheck();

        boost::dynamic_bitset<> getBitset();

        boost::uuids::uuid getId();

        int getOwner();

        char getCharRep() const;

        void setCharRep(char c);

        Location getLocation() const;

        int getLocationIndex() const;

        void setLocation(Location loc, int index);

        void setAppliesCheck(bool val);
    };
}
