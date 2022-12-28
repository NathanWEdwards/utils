//! MIT license.

use serde::ser::SerializeStruct;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Sequence {
    start: usize,
    end: usize,
}

impl Sequence {
    pub fn new(start: usize, end: usize) -> Sequence {
        Sequence {
            start: start,
            end: end,
        }
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>{
        Ok(serde_json::to_string(&self)?)
    }
}

impl serde::ser::Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer, 
    {
        let mut serialized_sequence = serializer.serialize_struct("Sequence", 2)?;
        serialized_sequence.serialize_field("start", &self.start)?;
        serialized_sequence.serialize_field("end", &self.end)?;
        serialized_sequence.end()
    }
}

pub struct Sequences {
    data: std::collections::BTreeMap<
        String,
        std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, Sequence>>>,
    >,
}

impl Sequences {
    pub fn new() -> Sequences {
        Sequences {
            data: std::collections::BTreeMap::new(),
        }
    }

    /// Given a string to match on, this method returns a JSON string representing all matching positions.
    pub fn find(&self, string: &str) -> Vec<Sequence> {
        let mut matches: Vec<Sequence> = Vec::new();

        match self.data.get(string) {
            Some(map) => {
                for (_, entry) in map.lock().unwrap().iter() {
                    matches.push(entry.clone());
                }
            }
            None => {}
        };
        return matches;
    }

    /// Given a sequence of UTF-8 encoded characters, return a vector of sub-strings found in the sequence as a vector of Strings.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// let string: String = String::from("actgggact");
    /// let expected:Vec<&str> = vec!["a", "ac", "act", "actg", "actgg", "actggg", "actggga", "actgggac", "actgggact", "c", "ct", "g", "gg", "ggg", "t"];
    /// let result: Vec<String> = utils::seq::Sequences::from(string).get_tokens();
    /// assert_eq!(result, expected);
    /// ```
    pub fn get_tokens(&self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        for key in self.data.keys() {
            tokens.push(key.clone());
        }
        return tokens;
    }

    fn insert(&mut self, key: &String, sequence: Sequence) {
        let new_sequence = Sequence::new(sequence.start, sequence.end);
        match self.data.get_mut(key) {
            Some(value) => {
                let index: String = format!("{}{}", sequence.start, sequence.end);
                value.lock().unwrap().insert(index.clone(), sequence);
            }
            None => {
                self.data.insert(
                    String::from(key),
                    std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::from([
                        (
                            format!("{}{}", sequence.start, sequence.end).clone(),
                            new_sequence,
                        ),
                    ]))),
                );
            }
        }
    }
}

impl From<String> for Sequences {
    fn from(string: String) -> Self {
        let input_length: usize = string.len();
        let mut sequences: Sequences = Sequences::new();
        let mut am: Vec<Vec<Sequence>> =
            vec![vec![Sequence::new(0, 0); input_length + 1]; input_length + 1];
        // For each character in the sequence,
        for index_i in 1..=input_length {
            // Set the current character value.
            let current_sequence_value: &str = &string[index_i - 1..index_i];
            // Compare each character in the sequence to a character in the sequence.
            for index_j in index_i..=input_length {
                // Assign the previous character in the sequence to a variable.
                let previous_sequence_value: &str = &string[index_j - 1..index_j];
                // If the current character being evaluated matches the previous character,
                if current_sequence_value == previous_sequence_value {
                    if index_j - index_i == 0 {
                        // Add the single character's position as an entry to Sequences.
                        let key = String::from(&string[index_j - 1..index_j]);
                        // Insert the matching sequence into a B-Tree.
                        sequences.insert(&key, Sequence::new(index_j - 1, index_j));
                        // Add the accumulated sequence to the adjaceny matrix.
                        am[index_i][index_j].start = am[index_i - 1][index_j - 1].start;
                        am[index_i][index_j].end = am[index_i - 1][index_j - 1].end + 1;
                    } else if index_j - index_i == 1 && am[index_i - 1][index_j - 1].end == 0 {
                        // If a repeating character is encountered for the first time,
                        // Add the beginning character's index and the index of the current character
                        // to the adjacency matrix.
                        am[index_i][index_j].start = index_j - 2;
                        am[index_i][index_j].end = index_j;
                    } else {
                        // For all other sequence matches,
                        // Add the beginning sequences's index as an offset from the matched index at `index_j`
                        // and the ending sequence as `index_j`.
                        am[index_i][index_j].start = index_j
                            - (am[index_i - 1][index_j - 1].end
                                - am[index_i - 1][index_j - 1].start)
                            - 1;
                        am[index_i][index_j].end = index_j;
                        // Add the sequence excluding the very beginning of the start index.
                        if am[index_i][index_j].end - am[index_i][index_j].start > 1 {
                            let key: String = String::from(
                                &string[am[index_i][index_j].start + 1..am[index_i][index_j].end],
                            );
                            sequences.insert(
                                &key,
                                Sequence::new(
                                    am[index_i][index_j].start + 1,
                                    am[index_i][index_j].end,
                                ),
                            );
                        }
                    }
                    // Add the sequence as an entry to Sequences.
                    let key: String =
                        String::from(&string[am[index_i][index_j].start..am[index_i][index_j].end]);
                    sequences.insert(
                        &key,
                        Sequence::new(am[index_i][index_j].start, am[index_i][index_j].end),
                    );
                }
            }
        }
        return sequences;
    }
}

impl std::fmt::Display for Sequences {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text: String = String::new();
        for (key, value) in self.data.iter() {
            text.push_str(&format!("sequence: {}:\n", key));
            for (_, sequence) in value.lock().unwrap().iter() {
                text.push_str(&format!(
                    "\tstart: {}, end: {}\n",
                    sequence.start, sequence.end
                ));
            }
        }
        writeln!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tokens_on_duplicate_string() {
        let string: String = String::from("actact");
        let expected: Vec<&str> = vec!["a", "ac", "act", "acta", "actac", "actact", "c", "ct", "t"];
        let result: Vec<String> = Sequences::from(string).get_tokens();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tokens_on_empty_string() {
        let string: String = String::from("");
        let expected: Vec<&str> = vec![];
        let result: Vec<String> = Sequences::from(string).get_tokens();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tokens_on_multiple_repeating_substrings() {
        let string: String = String::from("actgggact");
        let expected: Vec<&str> = vec![
            "a",
            "ac",
            "act",
            "actg",
            "actgg",
            "actggg",
            "actggga",
            "actgggac",
            "actgggact",
            "c",
            "ct",
            "g",
            "gg",
            "ggg",
            "t",
        ];
        let result: Vec<String> = Sequences::from(string).get_tokens();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tokens_on_repeating_character() {
        let string = String::from("gggg");
        let expected: Vec<&str> = vec!["g", "gg", "ggg", "gggg"];
        let result: Vec<String> = Sequences::from(string).get_tokens();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tokens_on_no_repeating_characters() {
        let string: String = String::from("abcdefg");
        let expected: Vec<&str> = vec![
            "a", "ab", "abc", "abcd", "abcde", "abcdef", "abcdefg", "b", "c", "d", "e", "f", "g",
        ];
        let result: Vec<String> = Sequences::from(string).get_tokens();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_on_multiple_occurrences() {
        let string: String = String::from("actact");
        let expected: Vec<Sequence> = vec![Sequence::new(0,3), Sequence::new(3,6)]; 
        let mut result: Vec<Sequence> = Sequences::from(string).find("act");
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_on_no_matching_occurrences() {
        let string: String = String::from("actact");
        let expected: Vec<Sequence> = Vec::new();
        let result: Vec<Sequence> = Sequences::from(string).find("tt");
        assert_eq!(result, expected);
    }
}
