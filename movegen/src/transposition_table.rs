use std::mem;

#[derive(Debug)]
pub struct TranspositionTable<K, V> {
    index_bits: usize,
    entries: Box<[Option<(K, V)>]>,
}

impl<K, V> TranspositionTable<K, V>
where
    K: Copy + Eq,
    V: Copy,
    u64: From<K>,
{
    pub fn new(index_bits: usize) -> TranspositionTable<K, V> {
        debug_assert!(index_bits <= 64);
        TranspositionTable {
            index_bits,
            entries: vec![None; 2_usize.pow(index_bits as u32)].into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.iter().filter(|x| x.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|x| x.is_none())
    }

    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    pub fn contains_key(&self, k: &K) -> bool {
        let index = self.key_to_index(k);
        match self.entries[index] {
            Some(entry) => entry.0 == *k,
            None => false,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let index = self.key_to_index(k);
        match self.entries[index] {
            Some(ref entry) if entry.0 == *k => Some(&entry.1),
            _ => None,
        }
    }

    // std::collections::HashMap::insert returns Option<V>
    // This method returns (K, V), because the returned key can be different from k. Only the
    // indexes must be equal.
    pub fn insert(&mut self, k: K, value: V) -> Option<(K, V)> {
        let index = self.key_to_index(&k);
        let old_entry = self.entries[index];
        self.entries[index] = Some((k, value));
        old_entry
    }

    pub fn reserved_memory(&self) -> usize {
        mem::size_of_val(&*self.entries)
    }

    fn key_to_index(&self, k: &K) -> usize {
        (u64::from(*k) >> (64 - self.index_bits)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;
    use crate::position_history::PositionHistory;
    use crate::r#move::{Move, MoveType};
    use crate::square::Square;
    use crate::zobrist::Zobrist;

    #[test]
    fn insert_and_replace() {
        let mut tt = TranspositionTable::<u64, u64>::new(8);

        assert_eq!(false, tt.contains_key(&0));
        assert_eq!(None, tt.get(&0));
        assert_eq!(false, tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));

        let replaced = tt.insert(0, 0);
        assert_eq!(None, replaced);

        assert_eq!(true, tt.contains_key(&0));
        assert_eq!(Some(&0), tt.get(&0));
        assert_eq!(false, tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));

        let replaced = tt.insert(0, 1);
        assert_eq!(Some((0, 0)), replaced);

        assert_eq!(true, tt.contains_key(&0));
        assert_eq!(Some(&1), tt.get(&0));
        assert_eq!(false, tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));

        let replaced = tt.insert(1, 2);
        assert_eq!(Some((0, 1)), replaced);

        assert_eq!(false, tt.contains_key(&0));
        assert_eq!(None, tt.get(&0));
        assert_eq!(true, tt.contains_key(&1));
        assert_eq!(Some(&2), tt.get(&1));
    }

    #[test]
    fn position_with_zobrist() {
        let mut tt = TranspositionTable::<Zobrist, u64>::new(20);

        let mut pos_history = PositionHistory::new(Position::initial());
        let hash = pos_history.current_pos_hash();
        assert_eq!(false, tt.contains_key(&hash));
        assert_eq!(None, tt.get(&hash));
        let old_entry = tt.insert(hash, 0);
        assert_eq!(None, old_entry);
        assert_eq!(true, tt.contains_key(&hash));
        assert_eq!(Some(&0), tt.get(&hash));

        pos_history.do_move(Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        let hash = pos_history.current_pos_hash();
        assert_eq!(false, tt.contains_key(&hash));
        assert_eq!(None, tt.get(&hash));
        let old_entry = tt.insert(hash, 1);
        assert_eq!(None, old_entry);
        assert_eq!(true, tt.contains_key(&hash));
        assert_eq!(Some(&1), tt.get(&hash));

        pos_history.undo_last_move();
        let hash = pos_history.current_pos_hash();
        assert_eq!(true, tt.contains_key(&hash));
        assert_eq!(Some(&0), tt.get(&hash));
        let old_entry = tt.insert(hash, 2);
        assert_eq!(Some((hash, 0)), old_entry);
        assert_eq!(true, tt.contains_key(&hash));
        assert_eq!(Some(&2), tt.get(&hash));
    }

    #[test]
    fn is_empty_and_len_and_capacity() {
        let table_idx_bits: usize = 8;
        let capacity = 2_usize.pow(table_idx_bits as u32);

        let mut tt = TranspositionTable::<u64, u64>::new(table_idx_bits);

        assert_eq!(0, tt.len());
        assert_eq!(true, tt.is_empty());
        assert_eq!(capacity, tt.capacity());

        let _ = tt.insert(0, 0);
        assert_eq!(1, tt.len());
        assert_eq!(false, tt.is_empty());
        assert_eq!(capacity, tt.capacity());

        let _ = tt.insert(0, 1);
        assert_eq!(1, tt.len());
        assert_eq!(false, tt.is_empty());
        assert_eq!(capacity, tt.capacity());

        let _ = tt.insert(0xff00_0000_0000_0000, 2);
        assert_eq!(2, tt.len());
        assert_eq!(false, tt.is_empty());
        assert_eq!(capacity, tt.capacity());
    }
}
