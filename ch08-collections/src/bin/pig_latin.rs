// Exercise 2
// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay”
// is added, so “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

// TODO: generalise from word to strings (account for whitespace)

fn to_pig_latin(s: &str) -> String {
    let mut word = s.to_string();
    word.push('-');
    // Check if the first character is a consonant or a vowel
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    if is_vowel(&first_char) {
        word.push('h');
    } else {
        let consonant = word.remove(0);
        word.push(consonant);
    }
    word.push_str("ay");
    word
}

fn is_vowel(c: &char) -> bool {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[test]
fn main() {
    assert_eq!(to_pig_latin("first"), String::from("irst-fay"));
    assert_eq!(to_pig_latin("apple"), String::from("apple-hay"));
    assert_eq!(to_pig_latin("npple"), String::from("pple-nay"));
}
