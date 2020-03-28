//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "piece.h"
#include "bitsetutil.h"

namespace protochess_engine {
    Piece::Piece(bool appliesCheck, int owner, boost::uuids::uuid id, boost::dynamic_bitset<> bitset, char charRep,
                 Location loc, int index) :
            appliesCheck(appliesCheck),
            owner(owner),
            id(id),
            bitset(bitset),
            charRep(charRep),
            location(loc),
            locationIndex(index) {
    }

    Location Piece::getLocation() const {
        return location;
    }

    void Piece::setLocation(Location loc, int index) {
        locationIndex = index;
        location = loc;
        bitset.reset();
        bitset.set(index, true);
    }

    boost::dynamic_bitset<> Piece::getBitset() {
        return bitset;
    }

    char Piece::getCharRep() const {
        return charRep;
    }

    void Piece::setCharRep(char cA) {
        charRep = cA;
    }

    int Piece::getLocationIndex() const {
        return locationIndex;
    }

    int Piece::getOwner() {
        return owner;
    }

    boost::uuids::uuid Piece::getId() {
        return id;
    }

    bool Piece::getAppliesCheck() {
        return appliesCheck;
    }

    void Piece::setAppliesCheck(bool val) {
        appliesCheck = val;
    }

    bool Piece::getMovedBefore() {
        return movedBefore;
    }

    void Piece::setMovedBefore(bool newVal) {
        movedBefore = newVal;
    }

    void Piece::setLastMovedBefore(bool newVal) {
        lastMovedBefore = newVal;
    }

    bool Piece::getLastMovedBefore() {
        return lastMovedBefore;
    }
}

