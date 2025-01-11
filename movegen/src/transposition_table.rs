use std::{cmp, mem};

pub trait TtEntry: Default {
    fn is_valid(&self) -> bool;

    fn depth(&self) -> usize;

    fn age(&self) -> u8;

    fn prio(&self, other: &Self, age: u8) -> cmp::Ordering;
}

pub const ENTRIES_PER_BUCKET: usize = 4;

type Bucket<K, V> = [(K, V); ENTRIES_PER_BUCKET];

#[derive(Debug)]
pub struct TranspositionTable<K, V> {
    index_bits: usize,
    buckets: Box<[Bucket<K, V>]>,
    len: usize,
}

impl<K, V> TranspositionTable<K, V>
where
    K: Copy + Default + Eq,
    V: Copy + TtEntry,
    u64: From<K>,
{
    pub fn new(bytes: usize) -> TranspositionTable<K, V> {
        debug_assert!(bytes <= u64::MAX as usize);
        // Reserve memory for at least 2 buckets, so that at least one index bit
        // is used (even if bytes is 0)
        let max_num_buckets = cmp::max(2, bytes / Self::bucket_size());
        // The actual number of buckets must be a power of 2.
        let index_bits = 64 - max_num_buckets.leading_zeros() - 1;
        debug_assert!(index_bits <= 64);
        TranspositionTable {
            index_bits: index_bits as usize,
            buckets: vec![
                [(Default::default(), Default::default()); ENTRIES_PER_BUCKET];
                2_usize.pow(index_bits)
            ]
            .into_boxed_slice(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(
            self.len,
            self.buckets
                .iter()
                .map(|b| b.iter().filter(|(_, v)| v.is_valid()).count())
                .sum()
        );
        self.len
    }

    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(
            self.len == 0,
            self.buckets
                .iter()
                .all(|b| b.iter().all(|(_, v)| !v.is_valid()))
        );
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.buckets.len() * ENTRIES_PER_BUCKET
    }

    pub fn clear(&mut self) {
        self.buckets
            .fill([(Default::default(), Default::default()); ENTRIES_PER_BUCKET]);
        self.len = 0;
    }

    pub fn load_factor_permille(&self) -> u16 {
        debug_assert!(self.len() <= self.capacity());
        (1000 * self.len() / self.capacity()) as u16
    }

    pub fn contains_key(&self, k: &K) -> bool {
        let bucket_idx = self.key_to_index(k);
        for entry in self.buckets[bucket_idx] {
            if entry.1.is_valid() && entry.0 == *k {
                return true;
            }
        }
        false
    }

    // Return entry with matching key
    pub fn get(&self, k: &K) -> Option<&V> {
        let bucket_idx = self.key_to_index(k);
        for entry in &self.buckets[bucket_idx] {
            if entry.1.is_valid() && entry.0 == *k {
                return Some(&entry.1);
            }
        }
        None
    }

    // Inserts a value into the table
    //
    // Replacement scheme:
    // 1. Entry with the same hash value (will only be replaced if new prio <= old prio)
    // 2. None entry
    // 3. Least important entry (i.e. highest prio)
    //
    // Note:
    // std::collections::HashMap::insert returns Option<V>
    // This method returns Option<(K, V)>, because the returned key can be
    // different from k. Only the indexes must be equal.
    pub fn insert(&mut self, k: K, value: V) -> Option<(K, V)> {
        let bucket_idx = self.key_to_index(&k);
        let mut replaced_idx = None;
        let mut replaced = None;
        let bucket = &mut self.buckets[bucket_idx];
        for (i, entry) in bucket.iter_mut().enumerate() {
            if !entry.1.is_valid() {
                // Entry is invalid => replace it
                replaced_idx = Some(i);
                replaced = None;
                break;
            }
            if entry.0 == k {
                // Existing entry has the same hash value as the new one
                if let cmp::Ordering::Greater = value.prio(&entry.1, value.age()) {
                    // New prio > old prio => keep existing entry
                    return None;
                }
                replaced_idx = Some(i);
                replaced = Some(*entry);
                break;
            }
            // Existing and new entries have different hash values
            match replaced {
                None => {
                    replaced_idx = Some(i);
                    replaced = Some(*entry);
                }
                Some(rep) => {
                    if let cmp::Ordering::Greater = entry.1.prio(&rep.1, value.age()) {
                        replaced_idx = Some(i);
                        replaced = Some(*entry);
                    }
                }
            }
        }
        match replaced_idx {
            Some(rep_idx) => {
                self.buckets[bucket_idx][rep_idx] = (k, value);
                self.len += replaced.is_none() as usize;
                replaced
            }
            None => None,
        }
    }

    pub fn bucket_size() -> usize {
        mem::size_of::<(K, V)>() * ENTRIES_PER_BUCKET
    }

    pub fn reserved_memory(&self) -> usize {
        mem::size_of_val(&*self.buckets)
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

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct U64Entry {
        is_valid: bool,
        value: u64,
    }

    impl TtEntry for U64Entry {
        fn is_valid(&self) -> bool {
            self.is_valid
        }

        fn depth(&self) -> usize {
            self.value as usize
        }

        fn age(&self) -> u8 {
            (self.value % 128) as u8
        }

        fn prio(&self, other: &Self, age: u8) -> cmp::Ordering {
            let halfmoves_since_self = ((age as u16 + 128 - self.age() as u16) % 128) as u8;
            let halfmoves_since_other = ((age as u16 + 128 - other.age() as u16) % 128) as u8;
            match halfmoves_since_self.cmp(&halfmoves_since_other) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => self.value.cmp(&other.value).reverse(),
                cmp::Ordering::Greater => cmp::Ordering::Greater,
            }
        }
    }

    impl From<u64> for U64Entry {
        fn from(value: u64) -> Self {
            Self {
                is_valid: true,
                value,
            }
        }
    }

    #[test]
    fn new() {
        let bucket_size = TranspositionTable::<u64, U64Entry>::bucket_size();

        // Always reserve memory for at least two buckets
        let tt = TranspositionTable::<u64, U64Entry>::new(0);
        assert_eq!(2 * bucket_size, tt.reserved_memory());
        let tt = TranspositionTable::<u64, U64Entry>::new(bucket_size);
        assert_eq!(2 * bucket_size, tt.reserved_memory());
        let tt = TranspositionTable::<u64, U64Entry>::new(2 * bucket_size);
        assert_eq!(2 * bucket_size, tt.reserved_memory());
        let tt = TranspositionTable::<u64, U64Entry>::new(4 * bucket_size - 1);
        assert_eq!(2 * bucket_size, tt.reserved_memory());
        let tt = TranspositionTable::<u64, U64Entry>::new(4 * bucket_size);
        assert_eq!(4 * bucket_size, tt.reserved_memory());
        let tt = TranspositionTable::<u64, U64Entry>::new(4 * bucket_size + 1);
        assert_eq!(4 * bucket_size, tt.reserved_memory());

        // Don't reserve more memory than wanted (if it is enough for 2 entries)
        let tt = TranspositionTable::<u64, U64Entry>::new(1000);
        assert!(tt.reserved_memory() <= 1000);
        let tt = TranspositionTable::<u64, U64Entry>::new(2000);
        assert!(tt.reserved_memory() <= 2000);
    }

    #[test]
    fn insert_and_replace_and_clear() {
        let capacity = 8 * ENTRIES_PER_BUCKET;
        let entry_size = mem::size_of::<(u64, U64Entry)>();
        let mut tt = TranspositionTable::<u64, U64Entry>::new(capacity * entry_size);

        assert!(!tt.contains_key(&0));
        assert_eq!(None, tt.get(&0));
        assert!(!tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));
        assert_eq!(0, tt.len());
        assert!(tt.is_empty());
        assert_eq!(capacity, tt.capacity());
        assert_eq!(0, tt.load_factor_permille());

        for i in 0..ENTRIES_PER_BUCKET {
            let key = i as u64;
            let value = U64Entry::from(key);
            let replaced = tt.insert(key, value);
            assert_eq!(None, replaced);
        }

        for i in 0..ENTRIES_PER_BUCKET {
            let key = i as u64;
            let value = U64Entry::from(key);
            assert!(tt.contains_key(&key));
            assert_eq!(Some(&value), tt.get(&key));
        }

        assert_eq!(ENTRIES_PER_BUCKET, tt.len());
        assert!(!tt.is_empty());
        assert_eq!(capacity, tt.capacity());
        assert_eq!(
            (1000 * tt.len() / tt.capacity()) as u16,
            tt.load_factor_permille()
        );

        let inserted = ENTRIES_PER_BUCKET as u64;
        assert!(!tt.contains_key(&inserted));
        assert_eq!(None, tt.get(&inserted));

        let replaced = tt.insert(inserted, inserted.into());
        assert_eq!(Some((0, 0.into())), replaced);
        assert!(!tt.contains_key(&0));
        assert_eq!(None, tt.get(&0));
        assert!(tt.contains_key(&1));
        assert_eq!(Some(&1.into()), tt.get(&1));
        assert!(tt.contains_key(&inserted));
        assert_eq!(Some(&inserted.into()), tt.get(&inserted));

        let another_inserted = inserted + 1;
        let replaced = tt.insert(another_inserted, another_inserted.into());
        assert_eq!(Some((1, 1.into())), replaced);
        assert!(!tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));
        assert!(tt.contains_key(&another_inserted));
        assert_eq!(Some(&another_inserted.into()), tt.get(&another_inserted));

        let _ = tt.insert(0xff00_0000_0000_0000, 2.into());
        assert_eq!(ENTRIES_PER_BUCKET + 1, tt.len());
        assert!(!tt.is_empty());
        assert_eq!(capacity, tt.capacity());
        assert_eq!(
            (1000 * tt.len() / tt.capacity()) as u16,
            tt.load_factor_permille()
        );

        tt.clear();
        assert!(!tt.contains_key(&0));
        assert_eq!(None, tt.get(&0));
        assert!(!tt.contains_key(&1));
        assert_eq!(None, tt.get(&1));
        assert!(!tt.contains_key(&inserted));
        assert_eq!(None, tt.get(&inserted));
        assert_eq!(0, tt.len());
        assert!(tt.is_empty());
        assert_eq!(capacity, tt.capacity());
        assert_eq!(0, tt.load_factor_permille());
    }

    #[test]
    fn position_with_zobrist() {
        let capacity = 16 * ENTRIES_PER_BUCKET;
        let entry_size = mem::size_of::<Option<(Zobrist, u64)>>();
        let mut tt = TranspositionTable::<Zobrist, U64Entry>::new(capacity * entry_size);

        let mut pos_history = PositionHistory::new(Position::initial());
        let hash = pos_history.current_pos_hash();
        assert!(!tt.contains_key(&hash));
        assert_eq!(None, tt.get(&hash));
        let old_entry = tt.insert(hash, 0.into());
        assert_eq!(None, old_entry);
        assert!(tt.contains_key(&hash));
        assert_eq!(Some(&0.into()), tt.get(&hash));

        pos_history.do_move(Move::new(
            Square::E2,
            Square::E4,
            MoveType::DOUBLE_PAWN_PUSH,
        ));
        let hash = pos_history.current_pos_hash();
        assert!(!tt.contains_key(&hash));
        assert_eq!(None, tt.get(&hash));
        let old_entry = tt.insert(hash, 1.into());
        assert_eq!(None, old_entry);
        assert!(tt.contains_key(&hash));
        assert_eq!(Some(&1.into()), tt.get(&hash));

        pos_history.undo_last_move();
        let hash = pos_history.current_pos_hash();
        assert!(tt.contains_key(&hash));
        assert_eq!(Some(&0.into()), tt.get(&hash));
        let old_entry = tt.insert(hash, 0.into());
        assert_eq!(Some((hash, 0.into())), old_entry);
        assert!(tt.contains_key(&hash));
        assert_eq!(Some(&0.into()), tt.get(&hash));
    }

    #[test]
    fn replace_correct_entry() {
        let capacity = 8 * ENTRIES_PER_BUCKET;
        let entry_size = mem::size_of::<Option<(u64, U64Entry)>>();
        let mut tt = TranspositionTable::<u64, U64Entry>::new(capacity * entry_size);
        assert!(tt.is_empty());

        let replaced = tt.insert(0, 0.into());
        assert_eq!(None, replaced);
        let replaced = tt.insert(0, 1.into());
        assert_eq!(Some((0, 0.into())), replaced);
        assert_eq!(1, tt.len());

        let replaced = tt.insert(1, 1.into());
        assert_eq!(None, replaced);
        assert_eq!(2, tt.len());

        let replaced = tt.insert(0, 0.into());
        assert_eq!(Some((0, 1.into())), replaced);
        assert_eq!(2, tt.len());
    }
}
