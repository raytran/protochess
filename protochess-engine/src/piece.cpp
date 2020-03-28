//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include <utility>
#include <utility>
#include "piece.h"
#include "bitsetutil.h"

namespace protochess_engine {
    Piece::Piece(bool promotable, char promoteTo, bool appliesCheck, int owner, boost::uuids::uuid id,
                 boost::dynamic_bitset<> bitset, char charRep,
                 Location loc, int index) :
            promotable(promotable),
            promoteTo(promoteTo),
            appliesCheck(appliesCheck),
            owner(owner),
            id(id),
            bitset(std::move(std::move(bitset))),
            charRep(charRep),
            lastCharRep(charRep),
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

    bool Piece::getPromotable() {
        return promotable;
    }

    void Piece::setPromotable(bool newVal) {
        promotable = newVal;
    }


    char Piece::getPromoteTo() {
        return promoteTo;
    }

    void Piece::setPromoteTo(char newChar) {
        promoteTo = newChar;
    }

    char Piece::getLastCharRep() const {
        return lastCharRep;
    }

    void Piece::setLastCharRep(char c) {
        lastCharRep = c;
    }
}

