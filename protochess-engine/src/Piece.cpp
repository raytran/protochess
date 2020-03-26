//
// Created by raytran on 3/25/20.
//

#include "Piece.h"

Piece::Piece(boost::uuids::uuid id, boost::dynamic_bitset<> bitset, char charRep, Location loc, int index) :
        id(id),
        bitset(bitset),
        charRep(charRep),
        location(loc),
        locationIndex(index) {
}

Location Piece::getLocation() {
    return location;
}

void Piece::setLocation(Location loc, int index) {
    locationIndex = index;
    location = loc;
}

boost::dynamic_bitset<> Piece::getBitset() const {
    return bitset;
}

void Piece::setBitset(boost::dynamic_bitset<> bs) {
    bitset = bs;
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
