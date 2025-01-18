use rand::Rng;

/* generates indexes based on a given sequence without repeating the same index until the whole sequence gets consumed */
pub struct IndexGenerator {
    available_indexes_count: usize,
    is_entry_used: Vec<bool>,
}

impl IndexGenerator {
    pub fn create(total_indexes_count: usize) -> Self {
        IndexGenerator {
            is_entry_used: vec![false; total_indexes_count],
            available_indexes_count: total_indexes_count,
        }
    }

    pub fn generate(&mut self) -> Option<usize> {
        let mut resulting_index = None;

        if self.available_indexes_count > 0 {
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

    pub fn is_empty(&self) -> bool {
        self.available_indexes_count == 0
    }
}
