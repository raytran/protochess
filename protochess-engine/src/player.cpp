//
// Created by raytran on 3/24/20.
//

#include <iostream>
#include "player.h"
#include "bitsetUtil.h"

Player::Player() : name("Anon") {}

Player::Player(std::string name) : name(name) {}

std::string Player::getName() {
    return name;
}

void Player::setPieces(std::map<boost::uuids::uuid, Piece> pieceMap) {
    pieces = pieceMap;

    for (auto &x:pieces) {
        allPieces = boost::dynamic_bitset<>(x.second.getBitset());
        break;
    }

    for (auto &x:pieces) {
        allPieces |= boost::dynamic_bitset<>(x.second.getBitset());
    }
}

std::map<boost::uuids::uuid, Piece> &Player::getPieces() {
    return pieces;
}

const std::map<char, MovementPattern> &Player::getMovementMap() {
    return movementMap;
}

void Player::setMovementMap(std::map<char, MovementPattern> map) {
    movementMap = map;
}


const std::map<char, MovementPattern> &Player::getCaptureMap() {
    return captureMap;
}

void Player::setCaptureMap(std::map<char, MovementPattern> map) {
    captureMap = map;
}

boost::dynamic_bitset<> Player::getAllPiecesBitset() {
    return allPieces;
}

boost::uuids::uuid Player::getPieceIdAt(Location loc) {
    for (auto const &x:pieces) {
        if (x.second.getLocation() == loc) {
            return x.first;
        }
    }
    return {00000000 - 0000 - 0000 - 0000 - 000000000000};
}

void Player::update() {
    for (auto &x:pieces) {
        allPieces = boost::dynamic_bitset<>(x.second.getBitset());
        break;
    }

    for (auto &x:pieces) {
        allPieces |= boost::dynamic_bitset<>(x.second.getBitset());
    }
}

void Player::removePiece(boost::uuids::uuid id) {
    pieces.erase(id);
    update();
}

