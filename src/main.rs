mod word_families;

use word_families::WordFamilies;

use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    // Embed the dictionary at compile time
    let dictionary_str: &'static str = include_str!("../data/dictionary.txt");

    // Convert it to a vector of strings, one per line
    let all_words: Vec<String> = dictionary_str
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    println!("Loaded {} words.", all_words.len());


    loop {
        let word_lengths = build_word_length_set(&all_words);

        let game_word_length = get_user_word_length(&word_lengths);

        let mut guesses_remaining = get_user_number_of_guesses();

        let display_words_remaining = get_user_display_preference();

        let mut game_words = extract_words_of_length(&all_words, game_word_length);

        let mut used_letters: Vec<char> = vec![];
        let mut user_wins = false;

        //The Main Loop for one game
        while guesses_remaining > 0 {
            //Creating a custom word families structure to store word families and their respective
            //words. For each time the user guesses a value, a new set of families will be
            //generated.
            let word_families = partition_words(&game_words, &used_letters);

            //Setting the new set of game words based on the partitioning that just happened
            //The second index of the tuple returned by the retrieve family function is the
            //list of words in the family.
            let selected_family_tuple = retrieve_family(&word_families);
            game_words = selected_family_tuple.1;

            //The first index of the tuple the selected word family (AKA the word with only the
            //guessed letters shown.) The value is being stored in a variable for ease of use.
            let word_pattern = selected_family_tuple.0;

            //Printing the information for the current turn
            println!("Guesses Remaining: {}", guesses_remaining);

            if display_words_remaining {
                println!("Words Remaining: {}", game_words.len());
            }

            //Creating a formatted string with all the letters the user has guessed.
            let used_letters_string = used_letters
                .iter()
                .map(|&c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("Used Letters: {}", used_letters_string);

            //Print the name of the largest word family.
            //The other value
            println!("Word: {}", word_pattern);

            let user_guess = get_user_guess();

            if word_pattern == game_words[0] {
                user_wins = true;
                break;
            }

            if used_letters.contains(&user_guess) {
                println!("You already guessed that letter!");
            } else {
                used_letters.push(user_guess);

                //Sorting the list of used letters so that it is in alphabetical orders
                used_letters.sort();

                //Partitioning Again
                // Check if the new guess completes the word
                let new_word_families = partition_words(&game_words, &used_letters);
                let new_selected_family = retrieve_family(&new_word_families);
                let new_word_pattern = new_selected_family.0;

                if new_word_pattern == game_words[0] {
                    user_wins = true;
                    break;
                }

                guesses_remaining -= 1;
            }

            //Print Blank Lines for ease of seeing
            println!("\n");
        }

        if user_wins {
            println!("You Win! Congratulations!");
        } else {
            println!("You lose!");
        }
        println!("The word was {}.", game_words[0]);

        //Asking the user if they want to play again
        if !play_again_prompt() {
            break;
        }
    }
}

fn build_word_length_set(all_words: &[String]) -> HashSet<usize> {
    // Using a HashSet ensures uniqueness of word lengths
    // Taking all words, trimming out any extra space (just in case) and storing the unique lengths
    // in the hashSet.
    all_words.iter().map(|word| word.trim().len()).collect()
}

fn get_user_word_length(word_lengths: &HashSet<usize>) -> usize {
    let mut input = String::new();

    //Infinite Loop until the user inputs a valid length
    loop {
        print!("Enter the number of letters you would like in the guessing word: ");

        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        input.clear(); // Clear previous input_num

        //Getting user input
        io::stdin().read_line(&mut input).unwrap();

        //Parsing the input as an usize
        match input.trim().parse::<usize>() {
            Ok(input_num) => {
                if word_lengths.contains(&input_num) {
                    return input_num;
                } else {
                    println!(
                        "There are no words with a length of {}. \nPlease input a different length",
                        input_num
                    );
                }
            }
            Err(_) => {
                println!("You input was invalid. Please try again.");
            }
        }
    }
}

//Returns a 32-bit unsigned integer representing the number of guesses the user would like for the
//game.
fn get_user_number_of_guesses() -> u32 {
    let mut input = String::new();

    //Infinite Loop until user inputs a valid number of guesses (not negative)
    loop {
        print!("Enter the number of guesses you would like to have: ");

        //Again, these lines are needed for input to be taken in on the same line as output and
        //have an empty string for each guess
        io::stdout().flush().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        //Parsing as an u32
        match input.trim().parse::<u32>() {
            Ok(input_num) => {
                return input_num;
            }
            Err(_) => {
                println!("Your input was invalid. Please try again.");
            }
        }
    }
}

//Returns true if the user wants to display how many guesses they have left.
//This function gets input from the user and keeps asking the player until they have answered in a
//valid way.
fn get_user_display_preference() -> bool {
    let mut input = String::new();

    loop {
        print!("Would you like to display a running total of words remaining?: ");

        io::stdout().flush().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        //Checking the response
        match input.trim().to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

fn get_user_guess() -> char {
    let mut input = String::new();

    loop {
        print!("Your guess?: ");

        io::stdout().flush().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        // Trim whitespace and check if input is exactly one alphabetic character
        match input.trim() {
            s if s.len() == 1 && s.chars().all(|c| c.is_alphabetic()) => {
                return s.chars().next().unwrap(); // Return the character if it's valid
            }
            _ => {
                println!("Your input was invalid. Please enter exactly one letter (no symbols or numbers).");
            }
        }
    }
}

//Slice of a string array is passed in rather than a whole string array.
//Functionally, this makes no difference but under the hood it saves some memory.
fn extract_words_of_length(words: &[String], length: usize) -> Vec<String> {
    words
        .iter() // Create an iterator over the vector
        .filter(|s| s.len() == length) // Filter words by length
        .map(|s| s.to_string()) // Convert &str to String
        .collect() // Collect into a new vector
}

//Function that takes a list of words and partitions it into families.
fn partition_words(words: &[String], used_letters: &[char]) -> WordFamilies {
    let mut word_families = WordFamilies::new();

    for word in words.iter() {
        let word_pattern = generate_pattern(word, used_letters);

        word_families.add_family(&word_pattern);
        word_families.add_word(&word_pattern, word);
    }

    word_families
}

//Using the word, generate a string with guessed letters shown and not guessed letters as '_'
fn generate_pattern(word: &str, used_letters: &[char]) -> String {
    //String to store the pattern (also known as what the word family looks like)
    let mut family_pattern = String::new();

    //Loop through the word
    for letter in word.chars() {
        //If the used letters have the character of the word we are looking at, add that character
        //to the string.
        if used_letters.contains(&letter) {
            family_pattern.push(letter);
        //Otherwise show the character as a dash.
        } else {
            family_pattern.push('_');
        }
    }

    family_pattern
}

// Returns a tuple of the family pattern and the vector of words that belongs to the family with the most words.
fn retrieve_family(word_families: &WordFamilies) -> (String, Vec<String>) {
    // Option to store the key and value (pattern and word family) with the most words
    let mut largest_family: Option<(&String, &Vec<String>)> = None;

    // Iterate over the word families using the `.iter()` method of the custom structure
    for (pattern, family) in word_families.iter() {
        if largest_family.is_none() || family.len() > largest_family.unwrap().1.len() {
            largest_family = Some((pattern, family));
        }
    }

    // Return a clone of the largest family and its pattern, or an empty string and vector if no family exists
    if let Some((pattern, family)) = largest_family {
        (pattern.clone(), family.clone())
    } else {
        ("".to_string(), Vec::new())
    }
}

fn play_again_prompt() -> bool {
    let mut input = String::new();

    loop {
        print!("Would you like to play again?: ");

        io::stdout().flush().unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        //Checking the response
        match input.trim().to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}
