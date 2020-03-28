//
// Created by raytran on 3/24/20.
//

#include <iostream>
#include <utility>
#include <utility>
#include "player.h"

namespace protochess_engine {
    Player::Player(int playerNum) : playerNum(playerNum), name("Anon") {}

    Player::Player(int playerNum, std::string name) : playerNum(playerNum), name(std::move(std::move(name))) {}

    std::string Player::getName() {
        return name;
    }

    void Player::setPieces(std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap) {
        pieces = std::move(pieceMap);
        piecesApplyCheck = {};

        for (auto &x:pieces) {
            if (x.second->getAppliesCheck()) {
                piecesApplyCheck.insert({x.first, x.second});
            }
        }

        for (auto &x:pieces) {
            allPieces = boost::dynamic_bitset<>(x.second->getBitset());
            break;
        }

        for (auto &x:pieces) {
            allPieces |= boost::dynamic_bitset<>(x.second->getBitset());
        }
    }

    std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &Player::getPieces() {
        return pieces;
    }

    std::map<char, MovementPattern> &Player::getMovementMap() {
        return movementMap;
    }

    void Player::setMovementMap(std::map<char, MovementPattern> map) {
        movementMap = std::move(map);
    }


    std::map<char, MovementPattern> &Player::getCaptureMap() {
        return captureMap;
    }

    void Player::setCaptureMap(std::map<char, MovementPattern> map) {
        captureMap = std::move(map);
    }

    boost::dynamic_bitset<> Player::getAllPiecesBitset() {
        return allPieces;
    }

    std::shared_ptr<Piece> Player::getPieceAt(Location loc) {
        for (auto &x:pieces) {
            if (x.second->getLocation() == loc) {
                return x.second;
            }
        }
        return nullptr;
    }

    void Player::update() {
        for (auto &x:pieces) {
            allPieces = boost::dynamic_bitset<>(x.second->getBitset());
            break;
        }

        for (auto &x:pieces) {
            allPieces |= boost::dynamic_bitset<>(x.second->getBitset());
        }
    }

    void Player::removePiece(boost::uuids::uuid id) {
        pieces.erase(id);
        if (piecesApplyCheck.count(id) != 0) {
            piecesApplyCheck.erase(id);
        }
    }

    void Player::addPiece(const std::shared_ptr<Piece> &piece) {
        pieces.insert({piece->getId(), piece});
    }

    void Player::disableCastleRights() {
        canCastle_ = false;
    }

    bool Player::canCastle() {
        return canCastle_;
    }

    int Player::getPlayerNum() {
        return playerNum;
    }

    void Player::enableCastleRights() {
        canCastle_ = true;
    }

    std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &Player::getPiecesApplyCheck() {
        return piecesApplyCheck;
    }

}
