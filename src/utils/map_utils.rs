use chrono::Datelike;
use std::{collections::HashMap, hash::Hash};

/// Update the times stamp value of a hashmap with the latest time.
/// If the key is missing, the key and the value gets added into the provided hashmap.
pub(crate) fn update_with_recent<K, T>(entry: &mut HashMap<K, T>, key: K, datetime: &T)
where
    K: Eq + Hash,
    T: PartialOrd + Copy + Datelike,
{
    match entry.get(&key) {
        None => {
            entry.insert(key, *datetime);
        }
        Some(old_date) if old_date < datetime => {
            entry.insert(key, *datetime);
        }
        _ => {}
    }
}
