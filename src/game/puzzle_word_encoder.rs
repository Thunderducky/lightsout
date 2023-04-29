pub const LEVEL_CODE_WORDS: [&str; 32] = [
    "Apple",       // 0
    "Banana",      // 1
    "Cherry",      // 2
    "Date",        // 3
    "Fig",         // 4
    "Grape",       // 5
    "Honeydew",    // 6
    "Kiwi",        // 7
    "Lemon",       // 8
    "Mango",       // 9
    "Nectarine",   // 10
    "Olive",       // 11
    "Papaya",      // 12
    "Quince",      // 13
    "Raspberry",   // 14
    "Strawberry",  // 15
    "Tangerine",   // 16
    "Pineapple",   // 17
    "Vanilla",     // 18
    "Watermelon",  // 19
    "Yuzu",        // 20
    "Zucchini",    // 21
    "Almond",      // 22
    "Blueberry",   // 23
    "Cashew",      // 24
    "Dragonfruit", // 25
    "Elderberry",  // 26
    "Fennel",      // 27
    "Grapefruit",  // 28
    "Hazelnut",    // 29
    "Jujube",      // 30
    "Kumquat",     // 31
];
// Write a rust function that takes in a slice of 5 booleans, interprets them as a binary number, and returns the corresponding index from the list above
// For example, if the input is [true, false, true, false, true], the function should return "Cherry"
fn pick_word(input: &[bool]) -> String {
    let mut index = 0;
    for i in 0..input.len() {
        // Expects input to have a length of 5, will fix eventually
        if input[4 - i] {
            // Goes backwards so it feels more like the actual number
            index += 2usize.pow(i as u32);
        }
    }
    LEVEL_CODE_WORDS[index].to_string().clone()
}

// Use the pick word function to take a vector of 25 booleans and return their corresponding words concatenated together
pub fn encode_level_as_words(input: &[bool]) -> String {
    let mut output = String::new();
    for i in 0..5 {
        let val = &input[i * 5..(i + 1) * 5];
        let word = pick_word(val);
        output.push_str(&word);
    }
    output
}

#[allow(dead_code)]
pub fn code_word_to_index(word: &str) -> usize {
    for i in 0..LEVEL_CODE_WORDS.len() {
        if LEVEL_CODE_WORDS[i] == word {
            return i;
        }
    }
    panic!("Invalid word: {}", word);
}

// Q:Is there a VS CODE plugin that makes it easy to run individual rust tests?
// A: https://marketplace.visualstudio.com/items?itemName=swellaby.rust-test-adapter
// Write a test function that verifies that `encode` works correctly
#[cfg(test)]
mod tests {
    use crate::game::{puzzle_word_encoder::encode_level_as_words, tile_puzzle::TilePuzzle};

    #[test]
    fn test_encode() {
        // Pick a random level
        let mut level = TilePuzzle::new();
        level.easy_puzzle();
        // Encode it as a word
        let encoded = encode_level_as_words(&level.tile_values);
        // Check the encoded string is what we expect
        assert_eq!(encoded, "CashewTangerineAppleAppleApple");
    }

    #[test]
    fn test_code_word_to_index(){
        // Pick a random word
        let word = "Cashew";
        // Get the index of that word
        assert_eq!(24, super::code_word_to_index(word));
    }
}
