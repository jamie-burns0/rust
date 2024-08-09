use std::collections::{hash_map::Entry, HashMap};

use regex::Regex;

pub fn string_is_not_a_palindrome_candidate( data: Option<&str> ) -> bool {

    let d = data.map_or("", |s| s);

    if d.is_empty() {
        return true;
    }

    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    re.is_match( d )
}

pub fn to_character_frequency_map( data: &str ) -> HashMap<String,u32> {

    if data.len() == 0 {
        return HashMap::new();
    }

    let mut map = HashMap::<String, u32>::new();

    for c in data.chars() {
        match map.entry( c.to_string() ) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            },
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    return map;
}


pub fn map_is_not_a_palindrome_candidate( map: &HashMap<String,u32>) -> bool {
    if map.is_empty() {
        return true;
    }
    let unpaired_count = map.values().filter(|&&frequency| frequency % 2 == 1).count();
    unpaired_count > 1
}

pub fn character_frequency_map_to_palindrome( map: &HashMap<String,u32>) -> String {
    let palindrome_length: u32 = map.values().sum();

    let mut palindrome: Vec<char> = vec![' '; palindrome_length as usize];

    let mut position: usize = 0;

    let mut entries: Vec<(String, u32)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    entries.sort_by_key(|(key, _)| key.clone());

    for (character, mut frequency) in entries {
        let c = character.chars().next().unwrap();
        if frequency % 2 == 1 {
            palindrome[(palindrome_length / 2) as usize] = c;
            frequency -= 1;
        }

        for _ in 0..(frequency / 2) {
            palindrome[position as usize] = c;
            palindrome[(palindrome_length as usize - position - 1) as usize] = c;
            position += 1;
        }
    }

    return String::from_iter(palindrome.iter());
}


pub fn make_palindrome_from( data: &str ) -> Option<String> {

    if string_is_not_a_palindrome_candidate(Some(data)) {
        return None;
    }

    let frequency_character_map = to_character_frequency_map(data);

    if map_is_not_a_palindrome_candidate(&frequency_character_map) {
        return None;
    }

    return Some(character_frequency_map_to_palindrome(&frequency_character_map));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_not_a_palindrome_candidate() {
        assert_eq!(string_is_not_a_palindrome_candidate( Some("") ), true);
    }
}