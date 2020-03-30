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
        char lastCharRep;
        int owner;
        bool promotable;
        char promoteTo;
        bool lastMovedBefore = false;
        bool movedBefore = false;
        bool appliesCheck;
        boost::uuids::uuid id;
        boost::dynamic_bitset<> bitset;
        Location location;
        int locationIndex;
    public:
        Piece(
                bool promotable,
                char promoteTo,
                bool appliesCheck,
                int owner,
                boost::uuids::uuid id,
                boost::dynamic_bitset<> bitset,
                char charRep,
                Location loc,
                int locationIndex);

        bool getLastMovedBefore();

        void setLastMovedBefore(bool newVal);

        bool getMovedBefore();

        bool getPromotable();

        void setPromotable(bool newVal);

        char getPromoteTo();

        void setPromoteTo(char newChar);

        void setMovedBefore(bool newVal);

        bool getAppliesCheck();

        boost::dynamic_bitset<> getBitset();

        std::string getIdString();

        boost::uuids::uuid getId();

        int getOwner();

        char getCharRep() const;

        void setCharRep(char c);


        char getLastCharRep() const;

        void setLastCharRep(char c);

        Location getLocation() const;

        int getLocationIndex() const;

        void setLocation(Location loc, int index);

        void setAppliesCheck(bool val);
    };
}
