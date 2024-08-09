use std::collections::HashMap;

use palindrome_lib::palindrome_lib::character_frequency_map_to_palindrome;
use palindrome_lib::palindrome_lib::make_palindrome_from;
use palindrome_lib::palindrome_lib::map_is_not_a_palindrome_candidate;
use palindrome_lib::palindrome_lib::string_is_not_a_palindrome_candidate;
use palindrome_lib::palindrome_lib::to_character_frequency_map;

#[test]
fn none_string_is_not_a_palindrome_candidate() {
    assert_eq!(string_is_not_a_palindrome_candidate(None), true);
}

#[test]
fn empty_string_is_not_a_palindrome_candidate() {
    assert_eq!(string_is_not_a_palindrome_candidate(Some("")), true);
}

#[test]
fn alphabetic_string_is_a_palindrome_candidate() {
    assert_eq!(string_is_not_a_palindrome_candidate(Some("abc")), false);
}

#[test]
fn non_alphabetic_string_is_not_a_palindrome_candidate() {
    assert_eq!(string_is_not_a_palindrome_candidate(Some("abc123")), true);
}

#[test]
fn any_alphabetic_string_returns_a_character_frequency_map() {
    assert_eq!(
        to_character_frequency_map("abbccc"),
        HashMap::from([
            (String::from("a"), 1),
            (String::from("b"), 2),
            (String::from("c"), 3)
        ])
    );

    assert_eq!(
        to_character_frequency_map("a"),
        HashMap::from([(String::from("a"), 1)])
    );

    assert_eq!(
        to_character_frequency_map("aaaaaa"),
        HashMap::from([(String::from("a"), 6)])
    );
}

#[test]
fn an_empty_string_returns_an_empty_hashmap() {
    assert_eq!(to_character_frequency_map(""), HashMap::from([]));
}

#[test]
fn an_empty_character_frequency_map_is_not_a_valid_palindrome_candidate() {
    assert_eq!(map_is_not_a_palindrome_candidate(&HashMap::new()), true);
}

#[test]
fn character_frequency_map_with_more_than_one_unpaired_character_is_not_a_valid_palindrome_candidate(
) {
    assert_eq!(
        map_is_not_a_palindrome_candidate(&HashMap::from([
            (String::from("a"), 1),
            (String::from("b"), 1)
        ])),
        true
    );
}

#[test]
fn character_frequency_map_with_no_unpaired_characters_is_a_valid_palindrome_candidate() {
    assert_eq!(
        map_is_not_a_palindrome_candidate(&HashMap::from([
            (String::from("a"), 2),
            (String::from("b"), 2)
        ])),
        false
    );
}

#[test]
fn character_frequency_map_with_one_unpaired_characters_is_a_valid_palindrome_candidate() {
    assert_eq!(
        map_is_not_a_palindrome_candidate(&HashMap::from([
            (String::from("a"), 2),
            (String::from("b"), 1)
        ])),
        false
    );
    assert_eq!(
        map_is_not_a_palindrome_candidate(&HashMap::from([(String::from("a"), 1)])),
        false
    );
}

#[test]
fn valid_character_frequency_map_will_return_a_palindrome() {
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([(String::from("a"), 1)])),
        String::from("a")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([(String::from("a"), 2)])),
        String::from("aa")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([(String::from("a"), 3)])),
        String::from("aaa")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([
            (String::from("a"), 2),
            (String::from("b"), 1)
        ])),
        String::from("aba")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([
            (String::from("a"), 1),
            (String::from("b"), 2)
        ])),
        String::from("bab")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([
            (String::from("a"), 2),
            (String::from("b"), 3)
        ])),
        String::from("abbba")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([
            (String::from("a"), 2),
            (String::from("b"), 3),
            (String::from("c"), 4)
        ])),
        String::from("abccbccba")
    );
    assert_eq!(
        character_frequency_map_to_palindrome(&HashMap::from([
            (String::from("a"), 3),
            (String::from("b"), 4),
            (String::from("c"), 2)
        ])),
        String::from("abbcacbba")
    );
}

#[test]
fn palindrome_candidate_will_return_a_palindrome() {
    assert_eq!(
        make_palindrome_from("a"), 
        Some(String::from("a"))
    );
    assert_eq!(
        make_palindrome_from("aa"), 
        Some(String::from("aa"))
    );
    assert_eq!(
        make_palindrome_from("aaa"), 
        Some(String::from("aaa"))
    );
    assert_eq!(
        make_palindrome_from("aab"), 
        Some(String::from("aba"))
    );
    assert_eq!(
        make_palindrome_from("abb"), 
        Some(String::from("bab"))
    );
    assert_eq!(
        make_palindrome_from("aabbb"), 
        Some(String::from("abbba"))
    );
    assert_eq!(
        make_palindrome_from("aabbbcccc"), 
        Some(String::from("abccbccba"))
    );
    assert_eq!(
        make_palindrome_from("aaabbbbcc"), 
        Some(String::from("abbcacbba"))
    );
}

#[test]
fn non_palindrome_candidate_will_return_none() {
    assert_eq!(make_palindrome_from(""),None);
    assert_eq!(make_palindrome_from("ab"),None);
    assert_eq!(make_palindrome_from("abc"),None);
    assert_eq!(make_palindrome_from("aab2"),None);
    assert_eq!(make_palindrome_from("123"),None);
    assert_eq!(make_palindrome_from("..--"),None);
}
