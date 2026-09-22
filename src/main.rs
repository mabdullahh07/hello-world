fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);
}

// Utility method that takes a character and returns true if digit
// or upper case letter.
fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

// Utility method that takes a character and returns true if it is
// an upper case letter.
fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn q1_parser(text: String) -> bool {
    // Make String into char vector
    let characters_array: Vec<char> = text.chars().collect();

    // Must contain at least two characters
    if characters_array.len() < 2 {
        return false;
    }

    // First two characters must be uppercase letters
    if !is_uppercase_letter(characters_array[0])
        || !is_uppercase_letter(characters_array[1])
    {
        return false;
    }

    // Remaining characters must be uppercase letters or digits
    for (i, character) in characters_array.iter().enumerate() {
        if i >= 2 && !is_uppercase_or_digit(*character) {
            return false;
        }
    }

    true
}