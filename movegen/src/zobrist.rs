use crate::bitboard::Bitboard;
use crate::piece;
use crate::position::{CastlingRights, Position};
use crate::side::Side;
use crate::square::Square;
use std::ops::{BitXor, BitXorAssign};

// Indices
// 0-383: White pieces
// 384-767: Black pieces
// 768: Black to move
// 769 - 772: Castling rights (KQkq)
// 773 - 780: En passant file
const IDX_FIRST_BLACK_PIECE: usize = 384;
const IDX_SIDE_TO_MOVE: usize = 768;
const IDX_WHITE_KINGSIDE: usize = 769;
const IDX_WHITE_QUEENSIDE: usize = 770;
const IDX_BLACK_KINGSIDE: usize = 771;
const IDX_BLACK_QUEENSIDE: usize = 772;
const IDX_FIRST_EN_PASSANT_FILE: usize = 773;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Zobrist(u64);

impl Zobrist {
    pub fn new(pos: &Position) -> Self {
        let mut zobrist = Self(0);

        for side in &[Side::White, Side::Black] {
            for piece_type in &[
                piece::Type::Pawn,
                piece::Type::Knight,
                piece::Type::Bishop,
                piece::Type::Rook,
                piece::Type::Queen,
                piece::Type::King,
            ] {
                let mut squares = pos.piece_occupancy(*side, *piece_type);
                while squares != Bitboard::EMPTY {
                    let square = squares.square_scan_forward_reset();
                    zobrist.toggle_piece(Some(piece::Piece::new(*side, *piece_type)), square)
                }
            }
        }

        zobrist.toggle_side_to_move(pos.side_to_move());
        zobrist.toggle_castling_rights(pos.castling_rights());
        zobrist.toggle_en_passant_square(pos.en_passant_square());

        zobrist
    }

    pub fn toggle_piece(&mut self, piece: Option<piece::Piece>, square: Square) {
        if let Some(p) = piece {
            let side_idx = p.piece_side() as usize * IDX_FIRST_BLACK_PIECE;
            let piece_type_idx = p.piece_type() as usize * Square::NUM_SQUARES;
            let square_idx = square.idx();
            *self ^= Self::KEYS[side_idx + piece_type_idx + square_idx];
        }
    }

    pub fn toggle_side_to_move(&mut self, side: Side) {
        if side == Side::Black {
            *self ^= Self::KEYS[IDX_SIDE_TO_MOVE];
        }
    }

    pub fn toggle_castling_rights(&mut self, cr: CastlingRights) {
        if cr.contains(CastlingRights::WHITE_KINGSIDE) {
            *self ^= Self::KEYS[IDX_WHITE_KINGSIDE];
        }
        if cr.contains(CastlingRights::WHITE_QUEENSIDE) {
            *self ^= Self::KEYS[IDX_WHITE_QUEENSIDE];
        }
        if cr.contains(CastlingRights::BLACK_KINGSIDE) {
            *self ^= Self::KEYS[IDX_BLACK_KINGSIDE];
        }
        if cr.contains(CastlingRights::BLACK_QUEENSIDE) {
            *self ^= Self::KEYS[IDX_BLACK_QUEENSIDE];
        }
    }

    pub fn toggle_en_passant_square(&mut self, en_passant: Bitboard) {
        if en_passant != Bitboard::EMPTY {
            let file = en_passant.to_square().file();
            *self ^= Self::KEYS[IDX_FIRST_EN_PASSANT_FILE + file.idx()];
        }
    }
}

impl BitXor for Zobrist {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor<&Self> for Zobrist {
    type Output = Self;

    fn bitxor(self, rhs: &Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor for &Zobrist {
    type Output = Zobrist;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXor<Zobrist> for &Zobrist {
    type Output = Zobrist;

    fn bitxor(self, rhs: Zobrist) -> Self::Output {
        Zobrist(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Zobrist {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&Self> for Zobrist {
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= rhs.0;
    }
}

impl From<u64> for Zobrist {
    fn from(u: u64) -> Self {
        Zobrist(u)
    }
}

impl From<Zobrist> for u64 {
    fn from(z: Zobrist) -> Self {
        z.0
    }
}

impl Zobrist {
    const NUM_KEYS: usize = 781;
    const KEYS: [Zobrist; Self::NUM_KEYS] = [
        Zobrist(0x20ae25dad4e2bc53),
        Zobrist(0x58f938e12bbf69fb),
        Zobrist(0xb5b235d60cf8e231),
        Zobrist(0x09f71ee2b019a7da),
        Zobrist(0xdb337b32c88c1a5e),
        Zobrist(0x299d525e1e3f808b),
        Zobrist(0x32f94202562eae4d),
        Zobrist(0x818bd26fb744f4c9),
        Zobrist(0xd2425d301da3f76a),
        Zobrist(0xea28e92d17f80845),
        Zobrist(0x6fa6e4bd1e4c11cb),
        Zobrist(0x060b203fa5a591f6),
        Zobrist(0xa341f25026534986),
        Zobrist(0x30f37ac823547189),
        Zobrist(0x919623ae9e77f296),
        Zobrist(0x028c6675005323cf),
        Zobrist(0x696aebf8c059d793),
        Zobrist(0xbeaf25d85f51c674),
        Zobrist(0x69cb596a08776c96),
        Zobrist(0x6cdd835950253486),
        Zobrist(0x14ed91d5a9c9e039),
        Zobrist(0xe1349846801022a0),
        Zobrist(0x8da9d6460d14e3e4),
        Zobrist(0x024d1641a7beb562),
        Zobrist(0x37951ced911cfd99),
        Zobrist(0x9cde02f2ab930f3a),
        Zobrist(0x3a38d093cb4f805a),
        Zobrist(0x6ef1091df209d54c),
        Zobrist(0x7e6eab54a8eec4e0),
        Zobrist(0xf307a32f0ea50ff5),
        Zobrist(0xa910d9c0f2afd10c),
        Zobrist(0x1684ec63657f5030),
        Zobrist(0xc079030fe1044038),
        Zobrist(0xaad097f23ed9f07b),
        Zobrist(0xb7ae5b3fba04664d),
        Zobrist(0xd5b911b1066fa3ba),
        Zobrist(0x171c6106d1018cf0),
        Zobrist(0x8910226b71800268),
        Zobrist(0xb59f69ae6c0a45c8),
        Zobrist(0xc93292820e1d7eaa),
        Zobrist(0x2e5efa84d9071a4f),
        Zobrist(0x9352207fa140a0ec),
        Zobrist(0x7d38bd890680ebc4),
        Zobrist(0x54b7246ccdbce448),
        Zobrist(0xf851d1732d10465f),
        Zobrist(0xb1d714de07f88e06),
        Zobrist(0x05bd489c14474c73),
        Zobrist(0x809f5a0870c2d531),
        Zobrist(0xe828d932f7eb90ef),
        Zobrist(0x57558b8147e46c12),
        Zobrist(0x6f4a61e69554721d),
        Zobrist(0x56b8fc2429cc428c),
        Zobrist(0xa0f69a5fa0001a5f),
        Zobrist(0x4f952a70dd6fe363),
        Zobrist(0x98cb700177688b1e),
        Zobrist(0xd7f07d80e2fc528f),
        Zobrist(0x336c05de786d099d),
        Zobrist(0x9c1a08bda2358e29),
        Zobrist(0x21e93622383173ef),
        Zobrist(0x374cb7e8b57da9a7),
        Zobrist(0x51f733dea2235580),
        Zobrist(0x81d85823023a8ad0),
        Zobrist(0xce61a5838da97631),
        Zobrist(0x8e0f00b9365e4323),
        Zobrist(0xe0ba3f667528bc20),
        Zobrist(0x7018053cc3eb4322),
        Zobrist(0x8f165449f5a1d1a1),
        Zobrist(0x3ab06d05c0ae03d3),
        Zobrist(0x5b53270ac32fab88),
        Zobrist(0xfb1c07272ad355ae),
        Zobrist(0xad3587d78cd4cdb3),
        Zobrist(0x9bab3f1b69c575f4),
        Zobrist(0x4345d853ee6c91d5),
        Zobrist(0x9c41785ec2444b05),
        Zobrist(0x948499aa22ce7e92),
        Zobrist(0x2888be62fef2e0dd),
        Zobrist(0x695e07f58d8d130d),
        Zobrist(0x7497b5bb3bd96b05),
        Zobrist(0x107b76f862248bd7),
        Zobrist(0xc651f2c128d6eacc),
        Zobrist(0x5bb2c0ba9c0f4a5d),
        Zobrist(0xab1cf439d709bf4b),
        Zobrist(0x8bade9425bee8ec1),
        Zobrist(0xf0c126dc9aa525e5),
        Zobrist(0xcc6be519cf587ddd),
        Zobrist(0x16902cbb17d5f7fb),
        Zobrist(0x537f60990ed2e949),
        Zobrist(0x26c69fec32d8f784),
        Zobrist(0x3db34a3de5d6b45b),
        Zobrist(0x873ce2d2eb015d9d),
        Zobrist(0x3233fe0220b84ac7),
        Zobrist(0x09849a33b73d6b3d),
        Zobrist(0xa6407967b147eefa),
        Zobrist(0x4d91a97db43ccc4b),
        Zobrist(0x096e9f3b18a64610),
        Zobrist(0x2f0be0d29a7b03d0),
        Zobrist(0x7ea6c091a7268c69),
        Zobrist(0x2bcee59c9298a2e5),
        Zobrist(0x92fabe895d9a99a4),
        Zobrist(0x268433409d143f66),
        Zobrist(0x2030169957521ee8),
        Zobrist(0xa8bca212ed6cd02d),
        Zobrist(0x8217549672dc893f),
        Zobrist(0xed685db05f5d3c81),
        Zobrist(0xb554180bc2263e43),
        Zobrist(0x9d0c267e475ee72e),
        Zobrist(0x221dd9e74f5b489f),
        Zobrist(0xa6bc91f29c55baa5),
        Zobrist(0x373378d710428273),
        Zobrist(0xce5b549dc06671a3),
        Zobrist(0x2bef9fba196f28e0),
        Zobrist(0x97bc766f012486aa),
        Zobrist(0xcd0ee7b6195d98a3),
        Zobrist(0x0e7e5f8a6d024552),
        Zobrist(0x2924d667848387f6),
        Zobrist(0xd2f785ab2ba2a368),
        Zobrist(0x426cd16a9dfa2da1),
        Zobrist(0x07b7e06acd6e565b),
        Zobrist(0xc3df7aedf178ae75),
        Zobrist(0x70474305bd60fa4c),
        Zobrist(0xe990ae68a616d581),
        Zobrist(0x815840ad32630750),
        Zobrist(0x6c600c08fe079e2e),
        Zobrist(0x9ecf1fd83b785cd1),
        Zobrist(0xec80b18ba4962a4c),
        Zobrist(0x49445d6d0df6188e),
        Zobrist(0x4fafa381c18964e6),
        Zobrist(0x0e494d0dddaacafc),
        Zobrist(0x3c19e3ee8807909e),
        Zobrist(0x19b82e233deca8c0),
        Zobrist(0xf82783b16e8497fb),
        Zobrist(0xd76dbcf09de29271),
        Zobrist(0xd8067b9c048f3400),
        Zobrist(0x98edb938f6452e34),
        Zobrist(0xca24ff12a4b2ca4d),
        Zobrist(0x9368bcb80564b078),
        Zobrist(0x0da94fbdddb74390),
        Zobrist(0x587cf737e481744f),
        Zobrist(0xfb0ae7d709e2ddc9),
        Zobrist(0xa53415f6c043c261),
        Zobrist(0x297f8f6673c8dbc5),
        Zobrist(0x7c629178e717c42e),
        Zobrist(0xa98b444e9fc60d65),
        Zobrist(0xf05f7cccd98911e1),
        Zobrist(0x8a2d31e7e317023d),
        Zobrist(0xd3c778e33e983130),
        Zobrist(0x06cb110b61540e2e),
        Zobrist(0x9f9b771f4095dd93),
        Zobrist(0xed61e8941c2664cd),
        Zobrist(0x8cfe4d6578c34709),
        Zobrist(0x5ed3e5a5a07667da),
        Zobrist(0x49b4d1cb95b5cb83),
        Zobrist(0x3fc81ae8178f27b2),
        Zobrist(0x3467f6978a905de8),
        Zobrist(0x2e794d67adc4e121),
        Zobrist(0x9ebe917114724991),
        Zobrist(0xa447eb89f458112b),
        Zobrist(0x4c8574cc1241591a),
        Zobrist(0xfd9c959a02841d52),
        Zobrist(0x842b77418c63bffa),
        Zobrist(0x719a1854422eb0ad),
        Zobrist(0xa508bc41aa52ba26),
        Zobrist(0x8e675232b2964887),
        Zobrist(0xb119e16646a871fe),
        Zobrist(0x5d1edeabd922fa7f),
        Zobrist(0x5457d5e2eb0b15ca),
        Zobrist(0x0d5ccc8c96af4bad),
        Zobrist(0x4520e54748dcd689),
        Zobrist(0x92a7d781c2accf6a),
        Zobrist(0x18060f62b9b28250),
        Zobrist(0x58d9623c0675c9dd),
        Zobrist(0x50b4c15d7c1e7600),
        Zobrist(0x8b41b3a698692b50),
        Zobrist(0xd020a2d64152e389),
        Zobrist(0xc79079b85d00eaa8),
        Zobrist(0x9543abe0208cfdfe),
        Zobrist(0x860fed40f9ad23e8),
        Zobrist(0x3f7c15704daa27d9),
        Zobrist(0x08c2ce6e65a04a5e),
        Zobrist(0x91d4e2208cb92d79),
        Zobrist(0xbc447b8800c0b9e3),
        Zobrist(0xaf6a4d5f8b305d70),
        Zobrist(0xd679df8295c6035d),
        Zobrist(0x3cdf36be356da281),
        Zobrist(0x7fd71b0d61cf7b36),
        Zobrist(0xe450c2825f870db5),
        Zobrist(0x2e7ecc05193954c4),
        Zobrist(0xb41f0a939928da5e),
        Zobrist(0x66d9cf410da006f0),
        Zobrist(0x516696b3e8d1703d),
        Zobrist(0x7bb0f8bd851a031a),
        Zobrist(0x0e62b7b94f4a2127),
        Zobrist(0x2e82951f3ff98c48),
        Zobrist(0x3d543ca223405982),
        Zobrist(0xca2ebf73d474be67),
        Zobrist(0x37d9996448616616),
        Zobrist(0xdbf96803059828da),
        Zobrist(0xfdf0120ad3f4f72a),
        Zobrist(0x689f520f0a899288),
        Zobrist(0xa52ca98dfdaab97f),
        Zobrist(0xa946a631763335f6),
        Zobrist(0xd38a14667829f938),
        Zobrist(0x20f021cf3c2b6f85),
        Zobrist(0xc00593e99d872585),
        Zobrist(0x9f22dc66ebedc4fd),
        Zobrist(0x3a25041811bfaeaa),
        Zobrist(0xa5dd138387a2a2fb),
        Zobrist(0x61f50fee1f530930),
        Zobrist(0x9370fe4e69ea393e),
        Zobrist(0x5f0e705198347bae),
        Zobrist(0xf04d78bd5ccd7560),
        Zobrist(0x3efcbfc380e0003e),
        Zobrist(0xf9aa9fc05de4b2c4),
        Zobrist(0x4beb74fb920aaeca),
        Zobrist(0x499d10a06a0a5c24),
        Zobrist(0x8ad92e0278842032),
        Zobrist(0xcce034cd57cd1568),
        Zobrist(0xd903cd375ab8d02a),
        Zobrist(0x30a0e6692f2e0155),
        Zobrist(0xe5f19472c9ca5d6d),
        Zobrist(0x5df107f7229a02f0),
        Zobrist(0xa655a366134c2a81),
        Zobrist(0x4bf69b244e374ebc),
        Zobrist(0x27158dedda14abb8),
        Zobrist(0xb8f9cfc293e12b5c),
        Zobrist(0xa5119a492370c543),
        Zobrist(0xcec4dc987297fd9a),
        Zobrist(0xa9cb0df5316a431d),
        Zobrist(0x05677a47420a19ef),
        Zobrist(0x70b45666c0616c71),
        Zobrist(0x2ccf2aa0619739c8),
        Zobrist(0x99169208b4f0e1aa),
        Zobrist(0x1b96aea9adbe47f6),
        Zobrist(0x17da2bf99692869d),
        Zobrist(0xfb3e8089b4d8f33c),
        Zobrist(0x34f9ef441f72ba88),
        Zobrist(0x2d38be0184b1595b),
        Zobrist(0x0c48755787b9e099),
        Zobrist(0x2d509464cd46af2b),
        Zobrist(0xe4b339cf81d433a9),
        Zobrist(0x53aac798239aefc8),
        Zobrist(0x0df7bea42fa8ee27),
        Zobrist(0x477005056ccfaf1f),
        Zobrist(0xa3deb406887fdad6),
        Zobrist(0x4ce2f07b008cfac9),
        Zobrist(0xef861cb534bbd469),
        Zobrist(0x5cd7af41a5c8d5f4),
        Zobrist(0xedd570da5515988e),
        Zobrist(0x81d068cbb734f60b),
        Zobrist(0xec2c312052aa1d11),
        Zobrist(0x3735d57dd66746c6),
        Zobrist(0x4e0beda5b81309eb),
        Zobrist(0x84c7ab31f323a8b8),
        Zobrist(0xb9b1ae5855f7ac58),
        Zobrist(0x36263f8ad7900b60),
        Zobrist(0x1fcd69080c2ef6f5),
        Zobrist(0x360adb5b6ca28797),
        Zobrist(0x6a37248fdef35cfc),
        Zobrist(0xbf7a202dac467651),
        Zobrist(0xa69f521996c3fb71),
        Zobrist(0xa0b0f4cfcfdb735e),
        Zobrist(0x4f9dbee472b837b5),
        Zobrist(0x45e8b24f669af2d6),
        Zobrist(0x2861299f0771a1ca),
        Zobrist(0x3a562e77c2bad075),
        Zobrist(0xc76b37911d2827dd),
        Zobrist(0xf33afffbd79a1e2c),
        Zobrist(0xf32d43d6c1e20f91),
        Zobrist(0x39b41f93cf6ca8cd),
        Zobrist(0xddc11c3e2d492cf2),
        Zobrist(0xcaaa452031bdec5a),
        Zobrist(0xdacaf84e3dbffaff),
        Zobrist(0x389c291f375a68fa),
        Zobrist(0xa7f1f2753875d6c1),
        Zobrist(0xf136219b76f83e22),
        Zobrist(0xed2d50bfe14c1271),
        Zobrist(0xab1ff90794432914),
        Zobrist(0xa4db58d954a316cb),
        Zobrist(0xbd2c7b13e65d471a),
        Zobrist(0x51e33433cf2d4c4e),
        Zobrist(0xd78ef7b64d509fd0),
        Zobrist(0x4e303726622bc983),
        Zobrist(0xf8404067abfa504e),
        Zobrist(0x2adeb631ea749ffc),
        Zobrist(0x74b7bfb7f732fce8),
        Zobrist(0x0b2c7ef69a3c6fcb),
        Zobrist(0xef30288d3b9f3e2c),
        Zobrist(0xb795ec0b465b1775),
        Zobrist(0x5d61f9e67b89a8f8),
        Zobrist(0x97add29312df674f),
        Zobrist(0x7737d779a3fe3698),
        Zobrist(0x0783369e77f8d4f4),
        Zobrist(0xee89f9a70c6e524a),
        Zobrist(0x3bf11aa91de72ee5),
        Zobrist(0x86f5fc7251ccfaa4),
        Zobrist(0x38c6967e7075a38d),
        Zobrist(0x67a393387c855dbb),
        Zobrist(0x119734eea51b28eb),
        Zobrist(0x55fc37a0d88390e4),
        Zobrist(0xcb9ea3522ac0ee65),
        Zobrist(0xb4008e13e756b036),
        Zobrist(0xfa6e92bb36d134eb),
        Zobrist(0x4f4e31d5e4c9f1ae),
        Zobrist(0xfb8bcf500f4b6f80),
        Zobrist(0xca792124c0b571a0),
        Zobrist(0xb59f4d222c950754),
        Zobrist(0xda3f4042a1439dae),
        Zobrist(0x892204b9ee74a5ea),
        Zobrist(0xae6785b698ff41ef),
        Zobrist(0x2e43e05c8a0fb6bd),
        Zobrist(0x0205d1af3106c2ce),
        Zobrist(0x40638bd5310fb2b3),
        Zobrist(0x9f37d9f7e9098ca1),
        Zobrist(0x1e87c09c1bf632a9),
        Zobrist(0xed583f3d173f4956),
        Zobrist(0xd9eaee86f1878986),
        Zobrist(0x5a258f7ae2e0ffc4),
        Zobrist(0x0e40cc3ccd05783a),
        Zobrist(0x88c474082412cc87),
        Zobrist(0x3582fe8155919ff6),
        Zobrist(0x254d9b41936ea908),
        Zobrist(0x1f5fe4a42807db2c),
        Zobrist(0x354026aa51cefda5),
        Zobrist(0xca45b44a66b9b00a),
        Zobrist(0x344fe0e2a8bb8e2c),
        Zobrist(0xd369c11fa6c691f5),
        Zobrist(0xf3bdbc06bbe23952),
        Zobrist(0x8b0509a4eda5384e),
        Zobrist(0x2b04739f75c9cf6c),
        Zobrist(0x467375857ffa913a),
        Zobrist(0x02472cb9679519f8),
        Zobrist(0x061b3f3e75424952),
        Zobrist(0x55fc605ba6d02baa),
        Zobrist(0x27ef21e137e7d65b),
        Zobrist(0xd9429e14c5384908),
        Zobrist(0x559a1d20a9eb1f42),
        Zobrist(0xe37fe17d4cb82831),
        Zobrist(0x181af10793825447),
        Zobrist(0xfba208c45fa05740),
        Zobrist(0x57972a92296d5365),
        Zobrist(0x13f27019a6bd1808),
        Zobrist(0x2d55474440bd6002),
        Zobrist(0xd4a7b34f4fbeb8c1),
        Zobrist(0x84182ec826994703),
        Zobrist(0x05b6436b327ced6b),
        Zobrist(0xba7b46f589a5d9cd),
        Zobrist(0x4700608dae3517c2),
        Zobrist(0x0eabe2dd94e23679),
        Zobrist(0xb8e87b84aa59da58),
        Zobrist(0xc8613cc537e40752),
        Zobrist(0x037235625e36e279),
        Zobrist(0x7b6564062f25908b),
        Zobrist(0x5f97315321a99711),
        Zobrist(0x2a1d5c7d562f309e),
        Zobrist(0xebcb33c7aebdf813),
        Zobrist(0xd238c10bbc27caf8),
        Zobrist(0x0c8d30e0be0416ad),
        Zobrist(0xde8b4733c558a930),
        Zobrist(0x1384e95db796e78e),
        Zobrist(0x522f749f6644a10d),
        Zobrist(0x754837c12527742a),
        Zobrist(0x066af5deb38c5c15),
        Zobrist(0x6339844674deff28),
        Zobrist(0x38193d2145aa954b),
        Zobrist(0xd4b13c02f9c04845),
        Zobrist(0xd92e6935094c20da),
        Zobrist(0x4e6f5cfb0004fed1),
        Zobrist(0x154b2a257dc8e90d),
        Zobrist(0xaafad4d5905481e7),
        Zobrist(0x82e750baea321ec7),
        Zobrist(0x063dd6540c88e900),
        Zobrist(0x1e9c199e7f6e8df3),
        Zobrist(0x82c52e938a92ae01),
        Zobrist(0xb8d197e33aa61448),
        Zobrist(0xfbf6b0cac04d86e3),
        Zobrist(0x80b207c60539d6bf),
        Zobrist(0x6f8e2b9b1e2a0212),
        Zobrist(0x08cdaed5485137c1),
        Zobrist(0xbadcbd9169c23837),
        Zobrist(0x5b6434614e3543ea),
        Zobrist(0xdd9f6160dd0b3ca4),
        Zobrist(0x45d3970debbdac12),
        Zobrist(0x4b557e28aae7575b),
        Zobrist(0x7c1e2bd572d89bce),
        Zobrist(0x1c50639216402f81),
        Zobrist(0xbbdfab6e953ebefc),
        Zobrist(0xa75e416c518f6761),
        Zobrist(0x62ae16bd0f03761e),
        Zobrist(0x7f254b969924387f),
        Zobrist(0x12c3853d6fe53b11),
        Zobrist(0x6d23c882abc44160),
        Zobrist(0x91b8694b2cb9e741),
        Zobrist(0xfe52fea27779f23e),
        Zobrist(0xf30f7e6df19dda6d),
        Zobrist(0x15055049bed76a52),
        Zobrist(0x7e8cb79f8046e9d1),
        Zobrist(0xcf8209f77058144e),
        Zobrist(0x64b96f8518cb0d95),
        Zobrist(0x2722543efcfdf577),
        Zobrist(0xac4f4901c5b36c45),
        Zobrist(0x19b5bd318af0c2d3),
        Zobrist(0xfb41623498db9389),
        Zobrist(0x7ed96700659a5554),
        Zobrist(0x3cd30894d87f2245),
        Zobrist(0x5b5b1f08eb702b32),
        Zobrist(0xf80cbb474108dd3d),
        Zobrist(0x5c0bbbb4de906782),
        Zobrist(0x5313145f47924ed3),
        Zobrist(0x9746f49f83dcdbe6),
        Zobrist(0xb58313902fd7b0db),
        Zobrist(0xd00a07cdb47aeb84),
        Zobrist(0xae03fb0429dee9da),
        Zobrist(0xd71c9d6bf99bd039),
        Zobrist(0x213332546119bb5c),
        Zobrist(0xce4ec17dcb8eedb2),
        Zobrist(0x3206ab9c888c0042),
        Zobrist(0x4d1e28f35050d150),
        Zobrist(0x3718411feffc5af5),
        Zobrist(0x18574c6f54c7cca8),
        Zobrist(0xad6e84fd5a8ce346),
        Zobrist(0x7cf5931fb07171ae),
        Zobrist(0xfc2bbc1928d01a84),
        Zobrist(0xc33ea719ef07e67d),
        Zobrist(0x9ce5dc2fe7ccc09d),
        Zobrist(0x1cc0c1dc9cf60a1e),
        Zobrist(0x2b246ea64b6a4cfe),
        Zobrist(0x0b40e16eb225b547),
        Zobrist(0x2fd81b70c85b0321),
        Zobrist(0x76ce30b7aab9a184),
        Zobrist(0x0408daee7cc0c3bc),
        Zobrist(0x38ff7edd0ffb97ed),
        Zobrist(0x9bbce1db8ee26ac7),
        Zobrist(0x1d19e3f80756172f),
        Zobrist(0x565de7cbd9d6eec0),
        Zobrist(0x8207f9f1c1671cef),
        Zobrist(0xe171dbf2b374441e),
        Zobrist(0x5c6abd06e6913814),
        Zobrist(0x8105a57c5adc2a62),
        Zobrist(0x75466f3b4952bad8),
        Zobrist(0xabb6047fff75b2f1),
        Zobrist(0x09a441c18df331e5),
        Zobrist(0x016385757c8914a6),
        Zobrist(0x972ea2265d09d42a),
        Zobrist(0x9ea609367ac86274),
        Zobrist(0xae698183e420ed30),
        Zobrist(0x409b97cb2823f0e0),
        Zobrist(0x75b34e632e68828d),
        Zobrist(0x61c8b9d8aa06f8f2),
        Zobrist(0x66b012db2bc5066d),
        Zobrist(0x301bfe166e786304),
        Zobrist(0xc5c3e694afa61802),
        Zobrist(0x35b59ad9128a4a6e),
        Zobrist(0xead1bd5e1dd6c761),
        Zobrist(0x59b0bb8593637935),
        Zobrist(0xbada8c1183f699d8),
        Zobrist(0x9c34ab23d6632cb5),
        Zobrist(0x8baebc432aec3b14),
        Zobrist(0xbe7c58a456ce1f1c),
        Zobrist(0x6f82975face365e0),
        Zobrist(0x25a1ea1d80dba14e),
        Zobrist(0x9ed34164143e2981),
        Zobrist(0x76894dd9772f07f4),
        Zobrist(0xdb0e7e9526c06501),
        Zobrist(0x6592185339849713),
        Zobrist(0x13c1478fe4306133),
        Zobrist(0x8b321c5b8fa8de72),
        Zobrist(0x2c14f6f5a82b4b93),
        Zobrist(0xcd855200765d5c50),
        Zobrist(0x66d4236019d94d7f),
        Zobrist(0x5eada9033eaebdb4),
        Zobrist(0x2f44740aef3db6f6),
        Zobrist(0x10e03569493271e2),
        Zobrist(0x36a7aa5e785fa1a6),
        Zobrist(0x3d878193cab2c07b),
        Zobrist(0xca13117022eb80bb),
        Zobrist(0x5bc8973daf6f995a),
        Zobrist(0xff90ee81c5626db1),
        Zobrist(0xa1f7347a6faaeb73),
        Zobrist(0xec3ad754a884f7ac),
        Zobrist(0x3ba797c1e2ed5abb),
        Zobrist(0x98e7c7d1a857d01a),
        Zobrist(0xb1b8cf4da986a6e7),
        Zobrist(0x35c5422d44c97b04),
        Zobrist(0xbbec83335429d088),
        Zobrist(0xe5d01944dd3d7802),
        Zobrist(0x57ff6d669a383baf),
        Zobrist(0x50fbe0a1495c2b88),
        Zobrist(0xcad5f0b2c8f778bc),
        Zobrist(0x2deed8103a2d83bd),
        Zobrist(0x5b645e4fce14a406),
        Zobrist(0x7cd1f9e332c0b6b8),
        Zobrist(0xf145533d31b131be),
        Zobrist(0x15a4c6591725c809),
        Zobrist(0x40bbb6790734c0dc),
        Zobrist(0x56abc623a36ca0ae),
        Zobrist(0xc32fa5dee113b790),
        Zobrist(0xd7a9acac6b9299fe),
        Zobrist(0x34d56995d36d8b88),
        Zobrist(0x9bcc2d60cc634a49),
        Zobrist(0x00568a631087bbaf),
        Zobrist(0x97af981929a0258c),
        Zobrist(0xe9320acc886801e6),
        Zobrist(0x5e1b1a6de818ca6d),
        Zobrist(0x105ee699a6ae3198),
        Zobrist(0x37e4cc64dc95ef4e),
        Zobrist(0x3b5240f6fbd47465),
        Zobrist(0x9333d9e56f512007),
        Zobrist(0xe41078b027a480f1),
        Zobrist(0x39cc9bf737dc73a8),
        Zobrist(0xa18e0c50bfd6c010),
        Zobrist(0xaa5987d6d6ab08bf),
        Zobrist(0x654ba8d58928b837),
        Zobrist(0x75796069a69cad23),
        Zobrist(0x986876b9b04c7024),
        Zobrist(0xf2a64d0f5f7f06cd),
        Zobrist(0xb4817dcbbd330091),
        Zobrist(0x7880d78eb0c974e1),
        Zobrist(0xf8458123f5bad8d5),
        Zobrist(0x632b6f87cd3ae72a),
        Zobrist(0x1396f9244d621c2d),
        Zobrist(0x8a0ee797a5f69223),
        Zobrist(0xce8edaf564ebdf21),
        Zobrist(0x5842c54fcfdb2e48),
        Zobrist(0xd161d4564a6b68f4),
        Zobrist(0xb04c4a009aeddaad),
        Zobrist(0x785d68c7283ac124),
        Zobrist(0x8a52d21f42de180a),
        Zobrist(0xdd972912129748c1),
        Zobrist(0x68b031f97136b662),
        Zobrist(0x1e1da58decb8cd00),
        Zobrist(0xcd841ae33c683c59),
        Zobrist(0xa408f1d6e2e6107d),
        Zobrist(0x66ecffc0f92f2f2a),
        Zobrist(0x52ad99181625f87c),
        Zobrist(0x2bc1051f17e3bb08),
        Zobrist(0x3113ac0a78a06dcd),
        Zobrist(0xcb06fd2aa969d4c0),
        Zobrist(0xe5ff80c51345a40e),
        Zobrist(0x85a7a962a062e153),
        Zobrist(0x5f1e3f739081fa1b),
        Zobrist(0xcfacb09a8ba6bcd8),
        Zobrist(0x8469ade4879328ef),
        Zobrist(0xfb282afc14611f56),
        Zobrist(0x8386426d524fe34b),
        Zobrist(0xf7c4d238fc3ef9de),
        Zobrist(0xf04fa5ab3776ee10),
        Zobrist(0x42683b8d1278d286),
        Zobrist(0xc5152b772a98310c),
        Zobrist(0xc6650d09ed588143),
        Zobrist(0xee20a3e8a4cbf5f8),
        Zobrist(0x6cfc90c4d100d5a7),
        Zobrist(0xf46481f51e99f859),
        Zobrist(0x852d8bc085268b6c),
        Zobrist(0xa15b0dd380b4f6bf),
        Zobrist(0x0e888ee2961e3d4e),
        Zobrist(0x5357b638176c3ad6),
        Zobrist(0xb2b92f52426a0734),
        Zobrist(0x98c6fca9fa64912c),
        Zobrist(0xac5d3473376e9781),
        Zobrist(0xee52a8e1b7a632f2),
        Zobrist(0xb0e6841fd58ddf6d),
        Zobrist(0x88dd4115f016b288),
        Zobrist(0xb438e64344d04f8e),
        Zobrist(0xb0a7833564588749),
        Zobrist(0x6f09e6487d27134d),
        Zobrist(0xd9a187e3e3334d3f),
        Zobrist(0xc174053ca5d1c60a),
        Zobrist(0x03f8346838990029),
        Zobrist(0x65d9681bc2f88ff4),
        Zobrist(0x77c2c93222bdce0d),
        Zobrist(0x1fbd6587e20fdf5b),
        Zobrist(0x2662a29b2f39f15b),
        Zobrist(0x1035a11f379ff14e),
        Zobrist(0x9e8e0966854ca755),
        Zobrist(0xca148ee5ff591786),
        Zobrist(0xbe4facaeca9f90cc),
        Zobrist(0xd3de09dc3d616674),
        Zobrist(0xab644d014a9eec2c),
        Zobrist(0x1bed1489d5afcc7e),
        Zobrist(0x527dff8c7342bfd5),
        Zobrist(0xd4c4c159adba1c8e),
        Zobrist(0x957aad5c7a47c934),
        Zobrist(0xbb497d30a15435f0),
        Zobrist(0xc713ce7a3f073682),
        Zobrist(0xd248082498e01358),
        Zobrist(0x447563a0f50d67aa),
        Zobrist(0x16610f2e061458da),
        Zobrist(0x78e73dd66bf189d1),
        Zobrist(0x142dc411ed9377cd),
        Zobrist(0x7a7e71683fe83df2),
        Zobrist(0x502348ace1954a64),
        Zobrist(0x7cee53619ffc76b8),
        Zobrist(0xb49da146ba7c3693),
        Zobrist(0xe259b348358f7ec3),
        Zobrist(0xc386a2f6d84fc64b),
        Zobrist(0xe3cc449eae97f16d),
        Zobrist(0x8ec04c2076d06e8c),
        Zobrist(0x9c647d8b901cc99e),
        Zobrist(0xaaa207da1ae175a4),
        Zobrist(0x186d88b725638404),
        Zobrist(0xfcf41b67ddfcd486),
        Zobrist(0xb0bda8e2c3eed440),
        Zobrist(0x9b4a871571e01731),
        Zobrist(0x52cac0188799bb0b),
        Zobrist(0x88b86944ee9a2d96),
        Zobrist(0xf96ea763e70951b9),
        Zobrist(0x5448cd503ea99b37),
        Zobrist(0x2762b58f0644bae3),
        Zobrist(0x3e156854067e6fe3),
        Zobrist(0xaa34ea2eecf62344),
        Zobrist(0x1c071b5ccdb968e3),
        Zobrist(0x1d3c38441179572b),
        Zobrist(0x95baa0133284800a),
        Zobrist(0xd43b32219cc33095),
        Zobrist(0x2584ae4418e9778f),
        Zobrist(0x22ee700a04cdf33e),
        Zobrist(0xe269678dac0cff2d),
        Zobrist(0x12946e9acb73c9fc),
        Zobrist(0x74f5b2a47a25a864),
        Zobrist(0xa14f9bb03573ff43),
        Zobrist(0xf172889d5aa6131b),
        Zobrist(0x1fed218117d29134),
        Zobrist(0xc418d4eab920cd10),
        Zobrist(0x4e601b5230170ddf),
        Zobrist(0x084a06b752c2ef32),
        Zobrist(0xbbb26d5ce6115c67),
        Zobrist(0x083805ed739297a7),
        Zobrist(0xe6e8a86870345481),
        Zobrist(0xc92ed0dfdffe2389),
        Zobrist(0x3e2e3fc81ec69410),
        Zobrist(0x39b7781de4257564),
        Zobrist(0x0458e6680866c9a4),
        Zobrist(0x32a2eae6d6c7cc08),
        Zobrist(0xa1f2c4b5b03f3e9e),
        Zobrist(0x6d3eafd7099193d8),
        Zobrist(0x8b26303943a331e6),
        Zobrist(0x25b78bb76f3c8e12),
        Zobrist(0x345413a9d78ba51a),
        Zobrist(0x59d35aaa27382bf7),
        Zobrist(0xe9572fc9bfd5c709),
        Zobrist(0xfe63d8d68e6ca463),
        Zobrist(0xaa14389a860c8e15),
        Zobrist(0x86792bbb867d6e10),
        Zobrist(0x12eee9eae23c76dd),
        Zobrist(0x45a1e95e7cf472de),
        Zobrist(0x33e0c3322fbb95d2),
        Zobrist(0x44d44f7d6c1bcf58),
        Zobrist(0x16dbdd905e47154e),
        Zobrist(0x59257bd471ba01ef),
        Zobrist(0xb33ebf2fc9858884),
        Zobrist(0x6aaf1f873992817a),
        Zobrist(0xa26820dadff736bb),
        Zobrist(0x9bf57ae473f21cc0),
        Zobrist(0x1dac2d616db44b27),
        Zobrist(0x025c046155746a43),
        Zobrist(0xcf252851778957bb),
        Zobrist(0xad293e6170d7bc3e),
        Zobrist(0x40f54ba5117b418e),
        Zobrist(0x21175fbc47b5c749),
        Zobrist(0xc88bc17f7293ae28),
        Zobrist(0xb3b16415bd401a51),
        Zobrist(0x7f351801d23a0751),
        Zobrist(0x8bb6e0ab83db4a32),
        Zobrist(0x37eca277f6340ae7),
        Zobrist(0x493686cf8edfa0d3),
        Zobrist(0x4afa39dbc6de8575),
        Zobrist(0xdafceb7a42a0f0d2),
        Zobrist(0x570c2e5ca17ef5bd),
        Zobrist(0xb33263810252eb6e),
        Zobrist(0x5a562ac2c2e7f3c1),
        Zobrist(0x53ce4074b6388249),
        Zobrist(0x5066cff8e4b0afd7),
        Zobrist(0x29847e4f350abd10),
        Zobrist(0x55037563924717b9),
        Zobrist(0x0d1280c6e06af671),
        Zobrist(0x11eed4508d1279a4),
        Zobrist(0x1e88d2d6dd76dab4),
        Zobrist(0xd6d1981aaa48b082),
        Zobrist(0xc027720f8dff43d8),
        Zobrist(0x064995acf98949b0),
        Zobrist(0x528d4ac0392cb3d0),
        Zobrist(0x9436645d8be882fa),
        Zobrist(0x8d43d93c80b8e5ef),
        Zobrist(0x09b66312ba8d6a35),
        Zobrist(0xcc0c16bc96f12ecd),
        Zobrist(0x35c1050e47029138),
        Zobrist(0x5a0e8742c5fe585d),
        Zobrist(0xbe368ebda631639b),
        Zobrist(0x4b5ca2c8300fb70e),
        Zobrist(0xbce8ef1dde9f56dc),
        Zobrist(0x4c412c70552561dc),
        Zobrist(0x813138f2ac9d745d),
        Zobrist(0x25630c4a515f6b2a),
        Zobrist(0xee9c861b970bd123),
        Zobrist(0x7c510cf8f6ea0838),
        Zobrist(0x5c3b68fa919c3552),
        Zobrist(0xb8c952dd5151eaf7),
        Zobrist(0xff46025acb6cc856),
        Zobrist(0x75e4dc6f2b0e693d),
        Zobrist(0xb7766b40ed4bd97c),
        Zobrist(0x489dd99964bd50b7),
        Zobrist(0xc6fe77b19a1bd161),
        Zobrist(0x8df6dbeac28249f2),
        Zobrist(0x4ab7798f527a0c8e),
        Zobrist(0x9105eca3eb309fbd),
        Zobrist(0x8e62c4e1154218dc),
        Zobrist(0xa7a3476ed8316f40),
        Zobrist(0xc6d5f569fe630a23),
        Zobrist(0xd6283f6927c4663b),
        Zobrist(0xe1b7201d396f835c),
        Zobrist(0x817a6b78a2ad8492),
        Zobrist(0x36e7d5a4afd76f1c),
        Zobrist(0x2c7e40099e0d2405),
        Zobrist(0x67c9d32fe294e8df),
        Zobrist(0x3750292b7e446d93),
        Zobrist(0x2afc7c1a553ac3ff),
        Zobrist(0x421a7d5a696b7918),
        Zobrist(0x9928a5a07f65f393),
        Zobrist(0xdef6ce474ebc4000),
        Zobrist(0x1207a845b2aefaad),
        Zobrist(0x9f5966aa801e31ba),
        Zobrist(0x1ff3120e4afeb1f0),
        Zobrist(0x4006c4b988a0da5a),
        Zobrist(0xc0e32696542a0014),
        Zobrist(0x7243d0a4454ff6a8),
        Zobrist(0x37362bf9cdd4510c),
        Zobrist(0xf0d79e7249ca6507),
        Zobrist(0x45f2d2d8a33d4989),
        Zobrist(0x900883aee10f17d5),
        Zobrist(0x981f315bd5cc5e1a),
        Zobrist(0xfab71d89be7680e6),
        Zobrist(0x0b905c26ef10db92),
        Zobrist(0x9677b012427d54fa),
        Zobrist(0x7ccd734e516337f1),
        Zobrist(0x056309ad9e447d2f),
        Zobrist(0x3058f6d402d24f97),
        Zobrist(0x0228be449c78e456),
        Zobrist(0x601125ae0cb39bf7),
        Zobrist(0xf41f2fda954277c0),
        Zobrist(0xb5ea0daf0ff6cc3b),
        Zobrist(0x3aa0eab7c8c87b05),
        Zobrist(0x56f1b673f9084d12),
        Zobrist(0xc9d0d9ca78a463a4),
        Zobrist(0x8b85cc4e7dfd8950),
        Zobrist(0x95637401f5c0002c),
        Zobrist(0xd5df84af394eb4aa),
        Zobrist(0xec1a4a46b19de5aa),
        Zobrist(0xe6b6422b4820a830),
        Zobrist(0xbbe3c56e93bed8e3),
        Zobrist(0x7a6eb91661eb5c7a),
        Zobrist(0x76a9a0dc76e95f5f),
        Zobrist(0xfbcb40988a69c0cf),
        Zobrist(0x56e8b44266851af4),
        Zobrist(0x42714f4a0ef71dc2),
        Zobrist(0x60941c522de99ce4),
        Zobrist(0x4494922e7626d180),
        Zobrist(0x83f0e421e8289aad),
        Zobrist(0xf68d8a0a02142eb8),
        Zobrist(0xa31662a8dc37be98),
        Zobrist(0x69241fbcd2b53a2e),
        Zobrist(0x2b99d1e7b80df365),
        Zobrist(0x42cee3323f833b63),
        Zobrist(0xfdcc833a96256e53),
        Zobrist(0x7aa44a5816ae676c),
        Zobrist(0x2373b9256ab4ee05),
        Zobrist(0xcf7ee13fff03ca22),
        Zobrist(0xc9125301bef9ae05),
        Zobrist(0x9dc3ede0ed5a7007),
        Zobrist(0x92a2d5697f28c754),
        Zobrist(0x729871b2305a5761),
        Zobrist(0xf6d5899c39a0a7fd),
        Zobrist(0x60a12a652afb4ac0),
        Zobrist(0xd30e0c10fd443cb6),
        Zobrist(0x97165e247010e28b),
        Zobrist(0x833a263cf7581aef),
        Zobrist(0x37e415f1f3afbf6f),
        Zobrist(0x4143205e0ae94479),
        Zobrist(0x1880649a955effa2),
        Zobrist(0x3cfa4d06e2403f7a),
        Zobrist(0x050b7e9132852fe5),
        Zobrist(0x78e5af54ce3a3b72),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::move_generator::MoveGenerator;
    use crate::position_history::PositionHistory;
    use crate::r#move::{Move, MoveList, MoveType};

    #[test]
    fn hash_values_differ() {
        let mut hash_values = Vec::new();

        let mut pos_history = PositionHistory::new(Position::initial());
        let mut move_list = MoveList::new();
        MoveGenerator::generate_moves(&mut move_list, pos_history.current_pos());
        for m in move_list.iter() {
            pos_history.do_move(*m);
            let hash = Zobrist::new(pos_history.current_pos());
            assert!(!hash_values.contains(&hash));
            hash_values.push(hash);
            pos_history.undo_last_move();
        }
    }

    #[test]
    fn hash_values_with_transposition_equal() {
        let mut hash_values = Vec::new();

        let mut pos_history = PositionHistory::new(Position::initial());
        let g2g3 = Move::new(Square::G2, Square::G3, MoveType::QUIET);
        let g1f3 = Move::new(Square::G1, Square::F3, MoveType::QUIET);
        let d7d5 = Move::new(Square::D7, Square::D5, MoveType::DOUBLE_PAWN_PUSH);

        pos_history.do_move(g2g3);
        pos_history.do_move(d7d5);
        pos_history.do_move(g1f3);
        let hash = Zobrist::new(pos_history.current_pos());
        assert!(!hash_values.contains(&hash));
        hash_values.push(hash);
        pos_history.undo_last_move();
        pos_history.undo_last_move();
        pos_history.undo_last_move();

        // Different move order
        pos_history.do_move(g1f3);
        pos_history.do_move(d7d5);
        pos_history.do_move(g2g3);
        let hash = Zobrist::new(pos_history.current_pos());
        assert!(hash_values.contains(&hash));
    }

    #[test]
    fn incremental_hash_values() {
        let mut pos_history = PositionHistory::new(Position::initial());
        let e2e4 = Move::new(Square::E2, Square::E4, MoveType::DOUBLE_PAWN_PUSH);
        let c7c5 = Move::new(Square::C7, Square::C5, MoveType::DOUBLE_PAWN_PUSH);

        pos_history.do_move(e2e4);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.do_move(c7c5);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.do_move(Move::NULL);
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());

        pos_history.undo_last_move();
        let hash = Zobrist::new(pos_history.current_pos());
        assert_eq!(hash, pos_history.current_pos_hash());
    }
}
