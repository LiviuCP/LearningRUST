use rand::Rng;

/* randomly generates indexes from a given sequence without repeating the same index until the whole sequence gets consumed */
pub trait IndexGenerator {
    fn generate(&mut self) -> Option<usize>;
    fn is_empty(&self) -> bool;
    fn create(total_indexes_count: usize) -> Self;
}

/* Stable generator, the relative ordering of indexes stays the same (but the algorithm is quadratic - to be used for small sized sequences only) */
pub struct StableIndexGenerator {
    available_indexes_count: usize,
    is_entry_used: Vec<bool>,
}

impl IndexGenerator for StableIndexGenerator {
    fn create(total_indexes_count: usize) -> Self {
        StableIndexGenerator {
            is_entry_used: vec![false; total_indexes_count],
            available_indexes_count: total_indexes_count,
        }
    }

    fn generate(&mut self) -> Option<usize> {
        let mut resulting_index = None;

        if !self.is_empty() {
            let generated_index =
                rand::thread_rng().gen_range(0..=self.available_indexes_count - 1);
            let mut found_available_indexes_count = 0;

            for i in 0..self.is_entry_used.len() {
                if !self.is_entry_used[i] {
                    found_available_indexes_count += 1;
                    if found_available_indexes_count - 1 == generated_index {
                        self.is_entry_used[i] = true;
                        self.available_indexes_count -= 1;
                        resulting_index = Some(i);
                        break;
                    }
                }
            }
        }

        resulting_index
    }

    fn is_empty(&self) -> bool {
        self.available_indexes_count == 0
    }
}

/* Efficient generator, generates indexes in constant time at the cost of modifying the relative ordering of indexes */
pub struct QuickIndexGenerator {
    indexes: Vec<usize>,
    low_pos: usize,
    high_pos: usize,
    available_indexes_count: usize,
}

impl QuickIndexGenerator {
    fn create_indexes_vector(indexes_count: usize) -> Vec<usize> {
        let mut indexes_vector = Vec::new();
        indexes_vector.reserve_exact(indexes_count);

        for index in 0..indexes_count {
            indexes_vector.push(index);
        }

        indexes_vector
    }
}

impl IndexGenerator for QuickIndexGenerator {
    fn create(total_indexes_count: usize) -> Self {
        QuickIndexGenerator {
            indexes: Self::create_indexes_vector(total_indexes_count),
            low_pos: 0,
            high_pos: if total_indexes_count > 0 {
                total_indexes_count - 1
            } else {
                0
            },
            available_indexes_count: total_indexes_count,
        }
    }

    fn generate(&mut self) -> Option<usize> {
        let mut resulting_index = None;

        if !self.is_empty() {
            let generated_relative_index_pos =
                rand::thread_rng().gen_range(0..=self.available_indexes_count - 1);
            let index_pos = self.low_pos + generated_relative_index_pos;
            resulting_index = Some(self.indexes[index_pos]);

            // shrink the useful (available indexes) part of the sequence by replacing the consumed index with the closest "border" index
            if self.high_pos > self.low_pos {
                let low_distance = index_pos - self.low_pos;
                let high_distance = self.high_pos - index_pos;

                if low_distance <= high_distance {
                    self.indexes[index_pos] = self.indexes[self.low_pos];
                    self.low_pos += 1;
                } else {
                    self.indexes[index_pos] = self.indexes[self.high_pos];
                    self.high_pos -= 1;
                }
            }

            self.available_indexes_count -= 1;
        }

        resulting_index
    }

    fn is_empty(&self) -> bool {
        self.available_indexes_count == 0
    }
}
