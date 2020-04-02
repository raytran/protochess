//
// Created by raytran on 4/1/20.
//
#pragma once
#include <boost/multiprecision/cpp_int.hpp>
#include <boost/functional/hash.hpp>
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

namespace protochess_engine{
    struct Dimensions{
        int width;
        int height;
    };
    using boost::multiprecision::uint1024_t;
    typedef uint1024_t bitboard;

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

    struct Piece{
        int owner;
        char charType;
    };

    struct Move{
        int fromIndex;
        int toIndex;
        bool isCapture;
        Piece captured;
        char promoteTo;
        //indicies must be 0<=index<1024
        Move(uint32_t fromIndx, uint32_t toIndx, char promotionType, bool isCapt, Piece capturedPiece){
            captured = capturedPiece;
            isCapture = isCapt;
            promoteTo = promotionType;
            fromIndex = fromIndx;
            toIndex = toIndx;
        }
        bool getIsCapture(){
            return isCapture;
        }
        Piece getCapturedPiece(){
            return captured;
        }
        int getFrom(){
            return fromIndex;
        }
        int getTo(){
            return toIndex;
        }
    };
    //@TODO switch to bitshifting based Move struct for performance
    /*
    struct Move{
        uint32_t data;
        bool isCapture;
        Piece captured;
        char promoteTo;
        //indicies must be 0<=index<1024
        Move(uint32_t fromIndex, uint32_t toIndex, char promotionType, bool isCapt, Piece capturedPiece){
            captured = capturedPiece;
            isCapture = isCapt;
            promoteTo = promotionType;
            data |= fromIndex << 20 | toIndex << 10;
        }
        bool getIsCapture(){
            return isCapture;
        }
        Piece getCapturedPiece(){
            return captured;
        }
        int getFrom(){
            return (data >> 20) & 0x3ff;
        }
        int getTo(){
            return (data >> 10) & 0x3ff;
        }
    };
     */
}
