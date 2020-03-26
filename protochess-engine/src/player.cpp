//
// Created by raytran on 3/24/20.
//

#include "player.h"

Player::Player() : name("Anon") {}

Player::Player(std::string name) : name(name) {}

std::string Player::getName() {
    return name;
}

void Player::setPieces(std::map<boost::uuids::uuid, Piece> pieceMap) {
    pieces = pieceMap;
}

const std::map<boost::uuids::uuid, Piece> &Player::getPieces() {
    return pieces;
}

const std::map<char, MovementPattern> &Player::getMovementMap() {
    return movementMap;
}

void Player::setMovementMap(std::map<char, MovementPattern> map) {
    movementMap = map;
}

