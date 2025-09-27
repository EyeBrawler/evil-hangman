use std::collections::HashMap;

//Creating structure to store the word family hash HashMap
pub struct WordFamilies {
    data: HashMap<String, Vec<String>>,
}

impl WordFamilies {
    //Creates a word family data structure
    pub fn new() -> Self {
        WordFamilies {
            data: HashMap::new(),
        }
    }

    pub fn add_family(&mut self, family_pattern: &str) {
        self.data
            .entry(family_pattern.to_string())
            .or_insert_with(Vec::new);
    }

    pub fn add_word(&mut self, family_pattern: &str, word: &str) {
        if let Some(list) = self.data.get_mut(family_pattern) {
            list.push(word.to_string());
        } else {
            println!("Word family '{}' does not exist!", family_pattern);
        }
    }

    // Provides an iterator over the key-value pairs in the HashMap
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<String>> {
        self.data.iter()
    }
}
