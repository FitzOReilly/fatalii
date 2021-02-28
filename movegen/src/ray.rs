use crate::bitboard::Bitboard;
use crate::direction::Direction;
use crate::square::Square;

pub struct Ray;

impl Ray {
    const RAYS: [[Bitboard; Square::NUM_SQUARES]; 8] = [
        Self::NORTH_RAYS,
        Self::SOUTH_RAYS,
        Self::EAST_RAYS,
        Self::WEST_RAYS,
        Self::NORTH_EAST_RAYS,
        Self::NORTH_WEST_RAYS,
        Self::SOUTH_EAST_RAYS,
        Self::SOUTH_WEST_RAYS,
    ];

    const NORTH_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x00000000000000fe),
        Bitboard(0x00000000000000fc),
        Bitboard(0x00000000000000f8),
        Bitboard(0x00000000000000f0),
        Bitboard(0x00000000000000e0),
        Bitboard(0x00000000000000c0),
        Bitboard(0x0000000000000080),
        Bitboard(0x0000000000000000),
        Bitboard(0x000000000000fe00),
        Bitboard(0x000000000000fc00),
        Bitboard(0x000000000000f800),
        Bitboard(0x000000000000f000),
        Bitboard(0x000000000000e000),
        Bitboard(0x000000000000c000),
        Bitboard(0x0000000000008000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000fe0000),
        Bitboard(0x0000000000fc0000),
        Bitboard(0x0000000000f80000),
        Bitboard(0x0000000000f00000),
        Bitboard(0x0000000000e00000),
        Bitboard(0x0000000000c00000),
        Bitboard(0x0000000000800000),
        Bitboard(0x0000000000000000),
        Bitboard(0x00000000fe000000),
        Bitboard(0x00000000fc000000),
        Bitboard(0x00000000f8000000),
        Bitboard(0x00000000f0000000),
        Bitboard(0x00000000e0000000),
        Bitboard(0x00000000c0000000),
        Bitboard(0x0000000080000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x000000fe00000000),
        Bitboard(0x000000fc00000000),
        Bitboard(0x000000f800000000),
        Bitboard(0x000000f000000000),
        Bitboard(0x000000e000000000),
        Bitboard(0x000000c000000000),
        Bitboard(0x0000008000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000fe0000000000),
        Bitboard(0x0000fc0000000000),
        Bitboard(0x0000f80000000000),
        Bitboard(0x0000f00000000000),
        Bitboard(0x0000e00000000000),
        Bitboard(0x0000c00000000000),
        Bitboard(0x0000800000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x00fe000000000000),
        Bitboard(0x00fc000000000000),
        Bitboard(0x00f8000000000000),
        Bitboard(0x00f0000000000000),
        Bitboard(0x00e0000000000000),
        Bitboard(0x00c0000000000000),
        Bitboard(0x0080000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0xfe00000000000000),
        Bitboard(0xfc00000000000000),
        Bitboard(0xf800000000000000),
        Bitboard(0xf000000000000000),
        Bitboard(0xe000000000000000),
        Bitboard(0xc000000000000000),
        Bitboard(0x8000000000000000),
        Bitboard(0x0000000000000000),
    ];

    const SOUTH_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000001),
        Bitboard(0x0000000000000003),
        Bitboard(0x0000000000000007),
        Bitboard(0x000000000000000f),
        Bitboard(0x000000000000001f),
        Bitboard(0x000000000000003f),
        Bitboard(0x000000000000007f),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000100),
        Bitboard(0x0000000000000300),
        Bitboard(0x0000000000000700),
        Bitboard(0x0000000000000f00),
        Bitboard(0x0000000000001f00),
        Bitboard(0x0000000000003f00),
        Bitboard(0x0000000000007f00),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000010000),
        Bitboard(0x0000000000030000),
        Bitboard(0x0000000000070000),
        Bitboard(0x00000000000f0000),
        Bitboard(0x00000000001f0000),
        Bitboard(0x00000000003f0000),
        Bitboard(0x00000000007f0000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000001000000),
        Bitboard(0x0000000003000000),
        Bitboard(0x0000000007000000),
        Bitboard(0x000000000f000000),
        Bitboard(0x000000001f000000),
        Bitboard(0x000000003f000000),
        Bitboard(0x000000007f000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000100000000),
        Bitboard(0x0000000300000000),
        Bitboard(0x0000000700000000),
        Bitboard(0x0000000f00000000),
        Bitboard(0x0000001f00000000),
        Bitboard(0x0000003f00000000),
        Bitboard(0x0000007f00000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000010000000000),
        Bitboard(0x0000030000000000),
        Bitboard(0x0000070000000000),
        Bitboard(0x00000f0000000000),
        Bitboard(0x00001f0000000000),
        Bitboard(0x00003f0000000000),
        Bitboard(0x00007f0000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0001000000000000),
        Bitboard(0x0003000000000000),
        Bitboard(0x0007000000000000),
        Bitboard(0x000f000000000000),
        Bitboard(0x001f000000000000),
        Bitboard(0x003f000000000000),
        Bitboard(0x007f000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0100000000000000),
        Bitboard(0x0300000000000000),
        Bitboard(0x0700000000000000),
        Bitboard(0x0f00000000000000),
        Bitboard(0x1f00000000000000),
        Bitboard(0x3f00000000000000),
        Bitboard(0x7f00000000000000),
    ];

    const EAST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0101010101010100),
        Bitboard(0x0202020202020200),
        Bitboard(0x0404040404040400),
        Bitboard(0x0808080808080800),
        Bitboard(0x1010101010101000),
        Bitboard(0x2020202020202000),
        Bitboard(0x4040404040404000),
        Bitboard(0x8080808080808000),
        Bitboard(0x0101010101010000),
        Bitboard(0x0202020202020000),
        Bitboard(0x0404040404040000),
        Bitboard(0x0808080808080000),
        Bitboard(0x1010101010100000),
        Bitboard(0x2020202020200000),
        Bitboard(0x4040404040400000),
        Bitboard(0x8080808080800000),
        Bitboard(0x0101010101000000),
        Bitboard(0x0202020202000000),
        Bitboard(0x0404040404000000),
        Bitboard(0x0808080808000000),
        Bitboard(0x1010101010000000),
        Bitboard(0x2020202020000000),
        Bitboard(0x4040404040000000),
        Bitboard(0x8080808080000000),
        Bitboard(0x0101010100000000),
        Bitboard(0x0202020200000000),
        Bitboard(0x0404040400000000),
        Bitboard(0x0808080800000000),
        Bitboard(0x1010101000000000),
        Bitboard(0x2020202000000000),
        Bitboard(0x4040404000000000),
        Bitboard(0x8080808000000000),
        Bitboard(0x0101010000000000),
        Bitboard(0x0202020000000000),
        Bitboard(0x0404040000000000),
        Bitboard(0x0808080000000000),
        Bitboard(0x1010100000000000),
        Bitboard(0x2020200000000000),
        Bitboard(0x4040400000000000),
        Bitboard(0x8080800000000000),
        Bitboard(0x0101000000000000),
        Bitboard(0x0202000000000000),
        Bitboard(0x0404000000000000),
        Bitboard(0x0808000000000000),
        Bitboard(0x1010000000000000),
        Bitboard(0x2020000000000000),
        Bitboard(0x4040000000000000),
        Bitboard(0x8080000000000000),
        Bitboard(0x0100000000000000),
        Bitboard(0x0200000000000000),
        Bitboard(0x0400000000000000),
        Bitboard(0x0800000000000000),
        Bitboard(0x1000000000000000),
        Bitboard(0x2000000000000000),
        Bitboard(0x4000000000000000),
        Bitboard(0x8000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
    ];

    const WEST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000001),
        Bitboard(0x0000000000000002),
        Bitboard(0x0000000000000004),
        Bitboard(0x0000000000000008),
        Bitboard(0x0000000000000010),
        Bitboard(0x0000000000000020),
        Bitboard(0x0000000000000040),
        Bitboard(0x0000000000000080),
        Bitboard(0x0000000000000101),
        Bitboard(0x0000000000000202),
        Bitboard(0x0000000000000404),
        Bitboard(0x0000000000000808),
        Bitboard(0x0000000000001010),
        Bitboard(0x0000000000002020),
        Bitboard(0x0000000000004040),
        Bitboard(0x0000000000008080),
        Bitboard(0x0000000000010101),
        Bitboard(0x0000000000020202),
        Bitboard(0x0000000000040404),
        Bitboard(0x0000000000080808),
        Bitboard(0x0000000000101010),
        Bitboard(0x0000000000202020),
        Bitboard(0x0000000000404040),
        Bitboard(0x0000000000808080),
        Bitboard(0x0000000001010101),
        Bitboard(0x0000000002020202),
        Bitboard(0x0000000004040404),
        Bitboard(0x0000000008080808),
        Bitboard(0x0000000010101010),
        Bitboard(0x0000000020202020),
        Bitboard(0x0000000040404040),
        Bitboard(0x0000000080808080),
        Bitboard(0x0000000101010101),
        Bitboard(0x0000000202020202),
        Bitboard(0x0000000404040404),
        Bitboard(0x0000000808080808),
        Bitboard(0x0000001010101010),
        Bitboard(0x0000002020202020),
        Bitboard(0x0000004040404040),
        Bitboard(0x0000008080808080),
        Bitboard(0x0000010101010101),
        Bitboard(0x0000020202020202),
        Bitboard(0x0000040404040404),
        Bitboard(0x0000080808080808),
        Bitboard(0x0000101010101010),
        Bitboard(0x0000202020202020),
        Bitboard(0x0000404040404040),
        Bitboard(0x0000808080808080),
        Bitboard(0x0001010101010101),
        Bitboard(0x0002020202020202),
        Bitboard(0x0004040404040404),
        Bitboard(0x0008080808080808),
        Bitboard(0x0010101010101010),
        Bitboard(0x0020202020202020),
        Bitboard(0x0040404040404040),
        Bitboard(0x0080808080808080),
    ];

    const NORTH_EAST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x8040201008040200),
        Bitboard(0x0080402010080400),
        Bitboard(0x0000804020100800),
        Bitboard(0x0000008040201000),
        Bitboard(0x0000000080402000),
        Bitboard(0x0000000000804000),
        Bitboard(0x0000000000008000),
        Bitboard(0x0000000000000000),
        Bitboard(0x4020100804020000),
        Bitboard(0x8040201008040000),
        Bitboard(0x0080402010080000),
        Bitboard(0x0000804020100000),
        Bitboard(0x0000008040200000),
        Bitboard(0x0000000080400000),
        Bitboard(0x0000000000800000),
        Bitboard(0x0000000000000000),
        Bitboard(0x2010080402000000),
        Bitboard(0x4020100804000000),
        Bitboard(0x8040201008000000),
        Bitboard(0x0080402010000000),
        Bitboard(0x0000804020000000),
        Bitboard(0x0000008040000000),
        Bitboard(0x0000000080000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x1008040200000000),
        Bitboard(0x2010080400000000),
        Bitboard(0x4020100800000000),
        Bitboard(0x8040201000000000),
        Bitboard(0x0080402000000000),
        Bitboard(0x0000804000000000),
        Bitboard(0x0000008000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0804020000000000),
        Bitboard(0x1008040000000000),
        Bitboard(0x2010080000000000),
        Bitboard(0x4020100000000000),
        Bitboard(0x8040200000000000),
        Bitboard(0x0080400000000000),
        Bitboard(0x0000800000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0402000000000000),
        Bitboard(0x0804000000000000),
        Bitboard(0x1008000000000000),
        Bitboard(0x2010000000000000),
        Bitboard(0x4020000000000000),
        Bitboard(0x8040000000000000),
        Bitboard(0x0080000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0200000000000000),
        Bitboard(0x0400000000000000),
        Bitboard(0x0800000000000000),
        Bitboard(0x1000000000000000),
        Bitboard(0x2000000000000000),
        Bitboard(0x4000000000000000),
        Bitboard(0x8000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
    ];

    const NORTH_WEST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000002),
        Bitboard(0x0000000000000004),
        Bitboard(0x0000000000000008),
        Bitboard(0x0000000000000010),
        Bitboard(0x0000000000000020),
        Bitboard(0x0000000000000040),
        Bitboard(0x0000000000000080),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000204),
        Bitboard(0x0000000000000408),
        Bitboard(0x0000000000000810),
        Bitboard(0x0000000000001020),
        Bitboard(0x0000000000002040),
        Bitboard(0x0000000000004080),
        Bitboard(0x0000000000008000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000020408),
        Bitboard(0x0000000000040810),
        Bitboard(0x0000000000081020),
        Bitboard(0x0000000000102040),
        Bitboard(0x0000000000204080),
        Bitboard(0x0000000000408000),
        Bitboard(0x0000000000800000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000002040810),
        Bitboard(0x0000000004081020),
        Bitboard(0x0000000008102040),
        Bitboard(0x0000000010204080),
        Bitboard(0x0000000020408000),
        Bitboard(0x0000000040800000),
        Bitboard(0x0000000080000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000204081020),
        Bitboard(0x0000000408102040),
        Bitboard(0x0000000810204080),
        Bitboard(0x0000001020408000),
        Bitboard(0x0000002040800000),
        Bitboard(0x0000004080000000),
        Bitboard(0x0000008000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000020408102040),
        Bitboard(0x0000040810204080),
        Bitboard(0x0000081020408000),
        Bitboard(0x0000102040800000),
        Bitboard(0x0000204080000000),
        Bitboard(0x0000408000000000),
        Bitboard(0x0000800000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0002040810204080),
        Bitboard(0x0004081020408000),
        Bitboard(0x0008102040800000),
        Bitboard(0x0010204080000000),
        Bitboard(0x0020408000000000),
        Bitboard(0x0040800000000000),
        Bitboard(0x0080000000000000),
        Bitboard(0x0000000000000000),
    ];

    const SOUTH_EAST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000100),
        Bitboard(0x0000000000010200),
        Bitboard(0x0000000001020400),
        Bitboard(0x0000000102040800),
        Bitboard(0x0000010204081000),
        Bitboard(0x0001020408102000),
        Bitboard(0x0102040810204000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000010000),
        Bitboard(0x0000000001020000),
        Bitboard(0x0000000102040000),
        Bitboard(0x0000010204080000),
        Bitboard(0x0001020408100000),
        Bitboard(0x0102040810200000),
        Bitboard(0x0204081020400000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000001000000),
        Bitboard(0x0000000102000000),
        Bitboard(0x0000010204000000),
        Bitboard(0x0001020408000000),
        Bitboard(0x0102040810000000),
        Bitboard(0x0204081020000000),
        Bitboard(0x0408102040000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000100000000),
        Bitboard(0x0000010200000000),
        Bitboard(0x0001020400000000),
        Bitboard(0x0102040800000000),
        Bitboard(0x0204081000000000),
        Bitboard(0x0408102000000000),
        Bitboard(0x0810204000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000010000000000),
        Bitboard(0x0001020000000000),
        Bitboard(0x0102040000000000),
        Bitboard(0x0204080000000000),
        Bitboard(0x0408100000000000),
        Bitboard(0x0810200000000000),
        Bitboard(0x1020400000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0001000000000000),
        Bitboard(0x0102000000000000),
        Bitboard(0x0204000000000000),
        Bitboard(0x0408000000000000),
        Bitboard(0x0810000000000000),
        Bitboard(0x1020000000000000),
        Bitboard(0x2040000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0100000000000000),
        Bitboard(0x0200000000000000),
        Bitboard(0x0400000000000000),
        Bitboard(0x0800000000000000),
        Bitboard(0x1000000000000000),
        Bitboard(0x2000000000000000),
        Bitboard(0x4000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
    ];

    const SOUTH_WEST_RAYS: [Bitboard; Square::NUM_SQUARES] = [
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000001),
        Bitboard(0x0000000000000002),
        Bitboard(0x0000000000000004),
        Bitboard(0x0000000000000008),
        Bitboard(0x0000000000000010),
        Bitboard(0x0000000000000020),
        Bitboard(0x0000000000000040),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000000100),
        Bitboard(0x0000000000000201),
        Bitboard(0x0000000000000402),
        Bitboard(0x0000000000000804),
        Bitboard(0x0000000000001008),
        Bitboard(0x0000000000002010),
        Bitboard(0x0000000000004020),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000000010000),
        Bitboard(0x0000000000020100),
        Bitboard(0x0000000000040201),
        Bitboard(0x0000000000080402),
        Bitboard(0x0000000000100804),
        Bitboard(0x0000000000201008),
        Bitboard(0x0000000000402010),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000001000000),
        Bitboard(0x0000000002010000),
        Bitboard(0x0000000004020100),
        Bitboard(0x0000000008040201),
        Bitboard(0x0000000010080402),
        Bitboard(0x0000000020100804),
        Bitboard(0x0000000040201008),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000000100000000),
        Bitboard(0x0000000201000000),
        Bitboard(0x0000000402010000),
        Bitboard(0x0000000804020100),
        Bitboard(0x0000001008040201),
        Bitboard(0x0000002010080402),
        Bitboard(0x0000004020100804),
        Bitboard(0x0000000000000000),
        Bitboard(0x0000010000000000),
        Bitboard(0x0000020100000000),
        Bitboard(0x0000040201000000),
        Bitboard(0x0000080402010000),
        Bitboard(0x0000100804020100),
        Bitboard(0x0000201008040201),
        Bitboard(0x0000402010080402),
        Bitboard(0x0000000000000000),
        Bitboard(0x0001000000000000),
        Bitboard(0x0002010000000000),
        Bitboard(0x0004020100000000),
        Bitboard(0x0008040201000000),
        Bitboard(0x0010080402010000),
        Bitboard(0x0020100804020100),
        Bitboard(0x0040201008040201),
    ];

    pub fn north_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::North)
    }

    pub fn north_east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::NorthEast)
    }

    pub fn east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::East)
    }

    pub fn south_east_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::positive_targets(origin, occupied, Direction::SouthEast)
    }

    pub fn south_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::South)
    }

    pub fn south_west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::SouthWest)
    }

    pub fn west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::West)
    }

    pub fn north_west_targets(origin: Square, occupied: Bitboard) -> Bitboard {
        Self::negative_targets(origin, occupied, Direction::NorthWest)
    }

    fn positive_targets(origin: Square, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = Self::RAYS[direction as usize][origin.idx()];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling square_scan_forward. Setting the most
        // significant bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x8000000000000000)).square_scan_forward();
        let targets = empty_board_targets ^ Self::RAYS[direction as usize][first_blocked.idx()];
        targets
    }

    fn negative_targets(origin: Square, occupied: Bitboard, direction: Direction) -> Bitboard {
        let empty_board_targets = Self::RAYS[direction as usize][origin.idx()];
        let blocked = empty_board_targets & occupied;
        // At least one bit must be set when calling square_scan_reverse. Setting the least
        // significant bit does not change the targets and allows a branchless implementation.
        let first_blocked = (blocked | Bitboard(0x0000000000000001)).square_scan_reverse();
        let targets = empty_board_targets ^ Self::RAYS[direction as usize][first_blocked.idx()];
        targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_targets() {
        assert_eq!(
            Bitboard::D2
                | Bitboard::D3
                | Bitboard::D4
                | Bitboard::D5
                | Bitboard::D6
                | Bitboard::D7
                | Bitboard::D8,
            Ray::north_targets(Square::D1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::D2 | Bitboard::D3 | Bitboard::D4 | Bitboard::D5 | Bitboard::D6 | Bitboard::D7,
            Ray::north_targets(Square::D1, Bitboard::D7)
        );

        assert_eq!(
            Bitboard::D2,
            Ray::north_targets(Square::D1, Bitboard::D2 | Bitboard::D7)
        );
    }

    #[test]
    fn south_targets() {
        assert_eq!(
            Bitboard::D7
                | Bitboard::D6
                | Bitboard::D5
                | Bitboard::D4
                | Bitboard::D3
                | Bitboard::D2
                | Bitboard::D1,
            Ray::south_targets(Square::D8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::D7 | Bitboard::D6 | Bitboard::D5 | Bitboard::D4 | Bitboard::D3 | Bitboard::D2,
            Ray::south_targets(Square::D8, Bitboard::D2)
        );

        assert_eq!(
            Bitboard::D7,
            Ray::south_targets(Square::D8, Bitboard::D7 | Bitboard::D2)
        );
    }

    #[test]
    fn east_targets() {
        assert_eq!(
            Bitboard::B4
                | Bitboard::C4
                | Bitboard::D4
                | Bitboard::E4
                | Bitboard::F4
                | Bitboard::G4
                | Bitboard::H4,
            Ray::east_targets(Square::A4, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B4 | Bitboard::C4 | Bitboard::D4 | Bitboard::E4 | Bitboard::F4 | Bitboard::G4,
            Ray::east_targets(Square::A4, Bitboard::G4)
        );

        assert_eq!(
            Bitboard::B4,
            Ray::east_targets(Square::A4, Bitboard::B4 | Bitboard::G4)
        );
    }

    #[test]
    fn west_targets() {
        assert_eq!(
            Bitboard::G4
                | Bitboard::F4
                | Bitboard::E4
                | Bitboard::D4
                | Bitboard::C4
                | Bitboard::B4
                | Bitboard::A4,
            Ray::west_targets(Square::H4, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G4 | Bitboard::F4 | Bitboard::E4 | Bitboard::D4 | Bitboard::C4 | Bitboard::B4,
            Ray::west_targets(Square::H4, Bitboard::B4)
        );

        assert_eq!(
            Bitboard::G4,
            Ray::west_targets(Square::H4, Bitboard::G4 | Bitboard::B4)
        );
    }

    #[test]
    fn north_east_targets() {
        assert_eq!(
            Bitboard::B2
                | Bitboard::C3
                | Bitboard::D4
                | Bitboard::E5
                | Bitboard::F6
                | Bitboard::G7
                | Bitboard::H8,
            Ray::north_east_targets(Square::A1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B2 | Bitboard::C3 | Bitboard::D4 | Bitboard::E5 | Bitboard::F6 | Bitboard::G7,
            Ray::north_east_targets(Square::A1, Bitboard::G7)
        );

        assert_eq!(
            Bitboard::B2,
            Ray::north_east_targets(Square::A1, Bitboard::B2 | Bitboard::G7)
        );
    }

    #[test]
    fn south_east_targets() {
        assert_eq!(
            Bitboard::B7
                | Bitboard::C6
                | Bitboard::D5
                | Bitboard::E4
                | Bitboard::F3
                | Bitboard::G2
                | Bitboard::H1,
            Ray::south_east_targets(Square::A8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::B7 | Bitboard::C6 | Bitboard::D5 | Bitboard::E4 | Bitboard::F3 | Bitboard::G2,
            Ray::south_east_targets(Square::A8, Bitboard::G2)
        );

        assert_eq!(
            Bitboard::B7,
            Ray::south_east_targets(Square::A8, Bitboard::B7 | Bitboard::G2)
        );
    }

    #[test]
    fn south_west_targets() {
        assert_eq!(
            Bitboard::G7
                | Bitboard::F6
                | Bitboard::E5
                | Bitboard::D4
                | Bitboard::C3
                | Bitboard::B2
                | Bitboard::A1,
            Ray::south_west_targets(Square::H8, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G7 | Bitboard::F6 | Bitboard::E5 | Bitboard::D4 | Bitboard::C3 | Bitboard::B2,
            Ray::south_west_targets(Square::H8, Bitboard::B2)
        );

        assert_eq!(
            Bitboard::G7,
            Ray::south_west_targets(Square::H8, Bitboard::G7 | Bitboard::B2)
        );
    }

    #[test]
    fn north_west_targets() {
        assert_eq!(
            Bitboard::G2
                | Bitboard::F3
                | Bitboard::E4
                | Bitboard::D5
                | Bitboard::C6
                | Bitboard::B7
                | Bitboard::A8,
            Ray::north_west_targets(Square::H1, Bitboard::EMPTY)
        );

        assert_eq!(
            Bitboard::G2 | Bitboard::F3 | Bitboard::E4 | Bitboard::D5 | Bitboard::C6 | Bitboard::B7,
            Ray::north_west_targets(Square::H1, Bitboard::B7)
        );

        assert_eq!(
            Bitboard::G2,
            Ray::north_west_targets(Square::H1, Bitboard::G2 | Bitboard::B7)
        );
    }
}
