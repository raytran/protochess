//
// Created by raytran on 4/5/20.
//
#pragma once

namespace protochess_engine {
    struct bitboard {
        unsigned long long b[4];

        bitboard() {
            b[0] = 0;
            b[1] = 0;
            b[2] = 0;
            b[3] = 0;
        }

        bitboard(unsigned long long d[4]) {
            b[0] = d[0];
            b[1] = d[1];
            b[2] = d[2];
            b[3] = d[3];
        }

        bitboard(unsigned long long d) {
            b[0] = d;
            b[1] = 0;
            b[2] = 0;
            b[3] = 0;
        }

        bitboard(unsigned long long d1, unsigned long long d2, unsigned long long d3, unsigned long long d4) {
            b[0] = d1;
            b[1] = d2;
            b[2] = d3;
            b[3] = d4;
        }

        bool operator==(const bitboard &rhs) const {
            return b[0] == rhs.b[0]
                   && b[1] == rhs.b[1]
                   && b[2] == rhs.b[2]
                   && b[3] == rhs.b[3];
        }

        bool operator!=(const bitboard &rhs) const {
            return b[0] != rhs.b[0]
                   || b[1] != rhs.b[1]
                   || b[2] != rhs.b[2]
                   || b[3] != rhs.b[3];
        }

        bitboard operator<<(int amt) const {
            boost::multiprecision::uint256_t data_lhs = 0;
            boost::multiprecision::import_bits(data_lhs, std::begin(b), std::end(b), 64, false);
            data_lhs = data_lhs << amt;
            std::vector<unsigned long long> v;
            boost::multiprecision::export_bits(data_lhs, std::back_inserter(v), 64, false);
            switch (v.size()) {
                case 4:
                    return bitboard(v[0], v[1], v[2], v[3]);
                case 3:
                    return bitboard(v[0], v[1], v[2], 0);
                case 2:
                    return bitboard(v[0], v[1], 0, 0);
                default:
                    return bitboard(v[0], 0, 0, 0);

            }
        }

        bitboard operator>>(int amt) const {
            boost::multiprecision::uint256_t data_lhs = 0;
            boost::multiprecision::import_bits(data_lhs, std::begin(b), std::end(b), 64, false);
            data_lhs = data_lhs >> amt;
            std::vector<unsigned long long> v;
            boost::multiprecision::export_bits(data_lhs, std::back_inserter(v), 64, false);
            switch (v.size()) {
                case 4:
                    return bitboard(v[0], v[1], v[2], v[3]);
                case 3:
                    return bitboard(v[0], v[1], v[2], 0);
                case 2:
                    return bitboard(v[0], v[1], 0, 0);
                default:
                    return bitboard(v[0], 0, 0, 0);

            }
        }

        bitboard operator|(const bitboard &rhs) const {
            return bitboard(
                    b[0] | rhs.b[0],
                    b[1] | rhs.b[1],
                    b[2] | rhs.b[2],
                    b[3] | rhs.b[3]
            );
        }

        bitboard operator&(const bitboard &rhs) const {
            return bitboard(
                    b[0] & rhs.b[0],
                    b[1] & rhs.b[1],
                    b[2] & rhs.b[2],
                    b[3] & rhs.b[3]
            );
        }

        bitboard operator^(const bitboard &rhs) const {
            return bitboard(
                    b[0] ^ rhs.b[0],
                    b[1] ^ rhs.b[1],
                    b[2] ^ rhs.b[2],
                    b[3] ^ rhs.b[3]
            );
        }

        bitboard operator~() const {
            return bitboard(
                    ~b[0],
                    ~b[1],
                    ~b[2],
                    ~b[3]
            );
        }

        void operator|=(const bitboard &rhs) {
            b[0] |= rhs.b[0];
            b[1] |= rhs.b[1];
            b[2] |= rhs.b[2];
            b[3] |= rhs.b[3];
        }

        void operator&=(const bitboard &rhs) {
            b[0] &= rhs.b[0];
            b[1] &= rhs.b[1];
            b[2] &= rhs.b[2];
            b[3] &= rhs.b[3];
        }

        void operator^=(const bitboard &rhs) {
            b[0] ^= rhs.b[0];
            b[1] ^= rhs.b[1];
            b[2] ^= rhs.b[2];
            b[3] ^= rhs.b[3];
        }

        operator bool() const {
            return (b[0] || b[1] || b[2] || b[3]);
        }
    };

    inline void reset(bitboard &bb) {
        bb.b[0] = 0;
        bb.b[1] = 0;
        bb.b[2] = 0;
        bb.b[3] = 0;
    }

    inline void bit_set(bitboard &bb, int index) {
        if (index < 64) {
            bb.b[0] |= (1UL << index);
        } else if (index < 128) {
            bb.b[1] |= (1UL << (index - 64));
        } else if (index < 192) {
            bb.b[2] |= (1UL << (index - 128));
        } else {
            bb.b[3] |= (1UL << (index - 192));
        }
    }

    inline void bit_unset(bitboard &bb, int index) {
        if (index < 64) {
            bb.b[0] &= ~(1UL << index);
        } else if (index < 128) {
            bb.b[1] &= ~(1UL << (index - 64));
        } else if (index < 192) {
            bb.b[2] &= ~(1UL << (index - 128));
        } else {
            bb.b[3] &= ~(1UL << (index - 192));
        }
    }

    inline bool bit_test(const bitboard &bb, int index) {
        if (index < 64) {
            return bb.b[0] & (1UL << index);
        } else if (index < 128) {
            return bb.b[1] & (1UL << (index - 64));
        } else if (index < 192) {
            return bb.b[2] & (1UL << (index - 128));
        } else {
            return bb.b[3] & (1UL << (index - 192));
        }
    }

    inline int lsb(const bitboard &bb) {
        if (bb.b[0]) {
            return __builtin_ffsll(bb.b[0]) - 1;
        } else if (bb.b[1]) {
            return 64 + __builtin_ffsll(bb.b[1]) - 1;
        } else if (bb.b[2]) {
            return 128 + __builtin_ffsll(bb.b[2]) - 1;
        } else if (bb.b[3]) {
            return 192 + __builtin_ffsll(bb.b[3]) - 1;
        }
        return 0;
    }

    inline int msb(const bitboard &bb) {
        if (bb.b[3]) {
            return 255 - __builtin_clzll(bb.b[3]);
        } else if (bb.b[2]) {
            return 191 - __builtin_clzll(bb.b[2]);
        } else if (bb.b[1]) {
            return 127 - __builtin_clzll(bb.b[1]);
        } else if (bb.b[0]) {
            return 63 - __builtin_clzll(bb.b[0]);
        }
        return 0;
    }
}
