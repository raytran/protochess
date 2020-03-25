//
// Created by raytran on 3/24/20.
//

#include "player.h"

Player::Player() : name("Anon") {}

Player::Player(std::string name) : name(name) {}

std::string Player::getName() {
    return name;
}

void Player::setPieces(std::map<char, boost::dynamic_bitset<>> pieceMap) {
    pieces = pieceMap;
}

std::map<char, boost::dynamic_bitset<>> Player::getPieces() const {
    return pieces;
}
