//! This module contains `Key` and `Track` types.

use interpolation::*;

#[derive(Debug, Clone, Copy, Deserialize)]
/// The `Key` Type.
pub struct Key {
    row: u32,
    value: f32,
    interpolation: Interpolation,
}

/// The `Track` Type. This is a collection of `Key`s with a name.
#[derive(Debug, Deserialize)]
pub struct Track {
    name: String,
    keys: Vec<Key>,
}

impl Key {
    /// Construct a new `Key`.
    pub fn new(row: u32, value: f32, interp: Interpolation) -> Key {
        Key {
            row: row,
            value: value,
            interpolation: interp,
        }
    }
}

impl Track {
    /// Construct a new Track with a name.
    pub fn new<S: Into<String>>(name: S) -> Track {
        Track {
            name: name.into(),
            keys: Vec::new(),
        }
    }

    /// Get the name of the track.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_exact_position(&self, row: u32) -> Option<usize> {
        self.keys.iter().position(|k| k.row == row)
    }

    fn get_insert_position(&self, row: u32) -> Option<usize> {
        match self.keys.iter().position(|k| k.row >= row) {
            Some(pos) => Some(pos),
            None => None,
        }
    }

    fn get_key_positions_for_row(&self, row: u32) -> (usize, usize) {
        if row < self.keys[0].row {
            (0,0)
        } else {
            match self.keys.iter().position(|k| row < k.row) {
                Some(pos) => ((pos - 1).max(0), pos),
                None => (self.keys.len() - 1, self.keys.len() - 1),
            }
        }
    }

    /// Insert or update a key on a track.
    pub fn set_key(&mut self, key: Key) {
        if let Some(pos) = self.get_exact_position(key.row) {
            self.keys[pos] = key;
        } else if let Some(pos) = self.get_insert_position(key.row) {
            self.keys.insert(pos, key);
        } else {
            self.keys.push(key);
        }
    }

    /// Delete a key from a track.
    ///
    /// If a key does not exist this will do nothing.
    pub fn delete_key(&mut self, row: u32) {
        if let Some(pos) = self.get_exact_position(row) {
            self.keys.remove(pos);
        }
    }

    /// Get a value based on a row.
    ///
    /// The row can be between two integers.
    /// This will perform the required interpolation.
    pub fn get_value(&self, row: f32) -> f32 {
        if self.keys.is_empty() {
            return 0.0;
        }

        let (lower_pos, higher_pos) = self.get_key_positions_for_row(row.floor() as u32);

        let lower = &self.keys[lower_pos];
        let higher = &self.keys[higher_pos];

        let t = (row - (lower.row as f32)) / ((higher.row as f32) - (lower.row as f32));
        let it = lower.interpolation.interpolate(t);

        (lower.value as f32) + ((higher.value as f32) - (lower.value as f32)) * it
    }
}
