use im_rc::HashMap;
use rustc_hash::{FxBuildHasher};

pub struct BoardPositions {
    persisted_positions: HashMap<u128, u8, FxBuildHasher>,
    most_frequent_position: Option<(u128, u8)>,
}

impl BoardPositions {
    pub fn empty() -> Self {
        Self {
            persisted_positions: HashMap::default(),
            most_frequent_position: None,
        }
    }

    pub fn persist_position(&mut self, zkey: &u128) {
        if !self.persisted_positions.contains_key(zkey) {
            self.persisted_positions.insert(*zkey, 0);
        }
        let occurrences_num = self.persisted_positions.get_mut(zkey).unwrap();
        *occurrences_num += 1;
        if let Some((_, old_occurrences)) = self.most_frequent_position.as_mut() {
            if occurrences_num > old_occurrences {
                self.most_frequent_position = Some((*zkey, *occurrences_num))
            }
        } else {
            self.most_frequent_position = Some((*zkey, *occurrences_num))
        }
    }

    pub fn most_frequent_position(&self) -> Option<&(u128, u8)> {
        self.most_frequent_position.as_ref()
    }
}
