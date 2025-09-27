# Evil Hangman (Rust)

An implementation of **Evil Hangman** written in Rust.  
In evil hangman, the computer doesn’t pick a word up front — it keeps changing the word pool to maximize difficulty until forced to commit.

## 📖 How It Works
- The game loads a dictionary of words from `data/dictionary.txt`.
- You choose:
  - The **word length**.
  - The **number of guesses**.
  - Whether you’d like to see the number of **possible words remaining**.
- After each guess:
  - The program partitions the remaining possible words into **families** (patterns of revealed letters and underscores).
  - It then **chooses the largest family**, keeping as many possibilities hidden as possible.
- You win if you guess the full word before running out of guesses.  
- If not, the game reveals the "final" word from the remaining family.
  
## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)

### Build & Run
Clone the repository and run the game:

```bash
git clone https://github.com/yourusername/evil-hangman-rust.git
cd evil-hangman-rust
cargo run
```

## 🎮 Example Gameplay
```
Loaded 127142 words.
Enter the number of letters you would like in the guessing word: 5
Enter the number of guesses you would like to have: 6
Would you like to display a running total of words remaining?: yes

Guesses Remaining: 6
Words Remaining: 1245
Used Letters:
Word: _____
Your guess?: e

Guesses Remaining: 5
Words Remaining: 643
Used Letters: e
Word: ___e_
...
```

## ✨ Features
* Dictionary-driven gameplay (easily swap out your own dictionary.txt).
* Dynamic word families to keep the game difficult.
* Input validation (no repeated guesses, only single alphabetic characters).
* Option to display the number of words still in play.
* Replay support.

## 📂 Dictionary
The game expects a file at:
```bash
data/dictionary.txt
```
This should be a plain text file with one word per line.
You can replace it with your own word list to customize gameplay.

## 🛠 Project Structure
```
.
├── src
│   ├── main.rs
│   └── word_families.rs
├── data
│   └── dictionary.txt
└── Cargo.toml
```

## 📜 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**.

You are free to use, modify, and distribute this software, but any derivative work must also be licensed under the GPL-3.0.

See the [LICENSE](LICENSE) file for the full text.