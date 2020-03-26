//
// Created by raytran on 3/25/20.
//

#include <iostream>
#include "Piece.h"
#include "bitsetUtil.h"

Piece::Piece(int owner, boost::uuids::uuid id, boost::dynamic_bitset<> bitset, char charRep, Location loc, int index) :
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

boost::dynamic_bitset<> &Piece::getBitset() {
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
