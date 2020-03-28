//
// Created by raytran on 3/24/20.
//

#pragma once

#include <vector>
#include <boost/uuid/uuid.hpp>
#include <boost/functional/hash.hpp>
#include <boost/dynamic_bitset.hpp>
#include <unordered_set>


namespace protochess_engine {
    struct Location {
        int x;
        int y;

    };

    inline bool operator==(const Location &lhs, const Location &rhs) {
        return lhs.x == rhs.x && lhs.y == rhs.y;
    }

    inline bool operator!=(const Location &lhs, const Location &rhs) {
        return !(lhs.x == rhs.x && lhs.y == rhs.y);
    }
}

namespace std {

    template<>
    struct hash<protochess_engine::Location> {
        std::size_t operator()(const protochess_engine::Location &m) const {
            using boost::hash_value;
            using boost::hash_combine;
            // Start with a hash value of 0    .
            std::size_t seed = 0;
            hash_combine(seed, hash_value(m.x));
            hash_combine(seed, hash_value(m.y));

            return seed;
        }
    };
}


namespace protochess_engine {
    struct Dimensions {
        int width;
        int height;
    };

    struct MovementPattern {
        bool north;
        bool east;
        bool south;
        bool west;
        bool northEast;
        bool northWest;
        bool southEast;
        bool southWest;
        ::std::unordered_set<Location> deltas;
    };

    struct LocationDelta {
        Location start;
        Location end;
    };

    inline bool operator==(const LocationDelta &lhs, const LocationDelta &rhs) {
        return lhs.start == rhs.start && lhs.end == rhs.end;
    }

    inline bool operator!=(const LocationDelta &lhs, const LocationDelta &rhs) {
        return !(lhs == rhs);
    }


    typedef boost::dynamic_bitset<>::size_type size_type;

    enum Direction {
        NORTH,
        EAST,
        SOUTH,
        WEST,
        NORTHEAST,
        NORTHWEST,
        SOUTHEAST,
        SOUTHWEST
    };

    //Forward declaration
    class Piece;
}


namespace protochess_engine {
    struct Move {
        bool promotion;
        char promotedType;
        bool capture;
        ::std::shared_ptr<Piece> targetPiece;
        bool castleKingSide;
        bool castleQueenSide;
        LocationDelta locationDelta;

        bool operator==(const Move &other) const {
            return (
                    promotion == other.promotion
                    && promotedType == other.promotedType
                    && capture == other.capture
                    && castleKingSide == other.castleKingSide
                    && castleQueenSide == other.castleQueenSide
                    && targetPiece == other.targetPiece
                    && locationDelta == other.locationDelta);
        }
    };
}

namespace std {

    template<>
    struct hash<protochess_engine::Move> {
        std::size_t operator()(const protochess_engine::Move &m) const {

            using boost::hash_value;
            using boost::hash_combine;

            // Start with a hash value of 0    .
            std::size_t seed = 0;

            // Modify 'seed' by XORing and bit-shifting in
            // one member of 'Key' after the other:
            hash_combine(seed, hash_value(m.targetPiece));
            hash_combine(seed, hash_value(m.capture));
            hash_combine(seed, hash_value(m.promotion));
            hash_combine(seed, hash_value(m.promotedType));
            hash_combine(seed, hash_value(m.castleKingSide));
            hash_combine(seed, hash_value(m.castleQueenSide));
            hash_combine(seed, hash_value(m.locationDelta.start.x));
            hash_combine(seed, hash_value(m.locationDelta.start.y));
            hash_combine(seed, hash_value(m.locationDelta.end.x));
            hash_combine(seed, hash_value(m.locationDelta.end.y));

            return seed;
        }
    };
}
