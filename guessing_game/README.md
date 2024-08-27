
# Guess The Number Game
Welcome to the "Guess The Number" game, a simple command-line game written in Rust. In this game, the player has a limited number of chances to guess a randomly generated number within a specified range.

## How to Play
1. **Set the Number of Chances:**  
   First, the game will ask you to enter the number of chances you want to guess the number.

2. **Set the Range:**  
   Next, you'll be asked to set the range of numbers (from 1 to your chosen number).

3. **Start Guessing:**  
   The game will then ask you to guess the number within the given range. After each guess, you'll be informed whether your guess was too small, too big, or correct.

4. **Winning and Losing:**  
   - If you guess the correct number within the allotted chances, you'll win!
   - If you run out of chances without guessing correctly, you'll lose.

## Example
Here's an example of how the game might proceed:

```bash
Game:: Guess The Number
Chance = 5
Range 1 to 100
<----------------------------------------------->
Input Your Guessing Number = 50
Left Chance = 4
Too small!
---------------------------------------------
Input Your Guessing Number = 75
Left Chance = 88
Too big!
---------------------------------------------
Input Your Guessing Number = 63
Left Chance = 57
Too small!
---------------------------------------------
Input Your Guessing Number = 69
Left Chance = 70
Too big!
---------------------------------------------
Input Your Guessing Number = 66
You Guessed = 66 😊
Congratulations, You win😊😊!
```
## Installation and Running
### Prerequisites
To run this game, you'll need to have Rust installed on your machine. If you don't have Rust installed, you can download it [here](https://www.rust-lang.org/tools/install).

### Clone the Repository
Clone this repository to your local machine:
```bash
  git clone https://github.com/bishnu7babu/my-rust-project.git
  cd guess-the-number
```
### Compile and Run
Compile the Rust program and run it:

```bash
  cargo run
```

## Customization
- You can change the number of chances and the range of numbers by modifying the game's input values.

- The game can be extended to provide more features, such as different difficulty levels, scoring, and more.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## Acknowledgments
- [Rust Programming Language](https://www.rust-lang.org/)
- [rand crate](https://crates.io/crates/rand)

Enjoy playing and have fun!
## contact
If you have any questions or suggestions, feel free to contact me at [your-email@example.com].
```bash
## Contact
If you have any questions or suggestions, feel free to contact me at [mbishnu7799@gmail.com].
```
This README provides a clear and concise guide to playing the game, along with instructions for setting up and running the project. You can adjust it based on your specific preferences or additional features you may want to add.
