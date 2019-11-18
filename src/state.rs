//! Has functions used to serialize and deserialize the program state
use crate::{State, STATE_FILE_NAME};
use serde_json::Result;
use std::fs;

/// Used to save any updates to the user's "state" (reviews on all chapters)
///
/// # Panics
/// Panics if: the file `rustacean-review-state` cannot be written to OR if the State struct is unable to be serialized.
pub fn serialize_state(data: &State) {
    let json = serde_json::to_string(data).unwrap();

    fs::write(STATE_FILE_NAME, json).unwrap();
}

/// Used to get saved state on program run.
/// Should not panic if file does not exist, returns `None` instead.
pub fn deserialize_state() -> Result<State> {
    let data = match fs::read_to_string(STATE_FILE_NAME) {
        Err(_) => String::from(""),
        Ok(string) => string,
    };

    serde_json::from_str(data.as_str())
}

#[cfg(test)]
mod tests {
    use crate::state::{deserialize_state, serialize_state};
    use crate::State;
    use std::collections::HashMap;

    #[test]
    #[ignore]
    fn serialize_and_deserialize() {
        let mut hashmap_example = HashMap::new();
        hashmap_example.insert(1, 2);
        hashmap_example.insert(3, 4);
        hashmap_example.insert(5, 6);

        let state = State {
            reviews: hashmap_example,
        };

        serialize_state(&state);

        let deserialized = deserialize_state().unwrap();

        assert_eq!(deserialized.reviews.len(), state.reviews.len());
        assert_eq!(deserialized.reviews.get(&1usize).unwrap(), &2u8);
        assert!(deserialized.reviews.contains_key(&5usize));
    }
}
