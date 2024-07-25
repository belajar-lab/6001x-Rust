## Introduction

For this problem, we will implement a variation of the classic wordgame Hangman. For those of you who are unfamiliar with the rules, you may read all about it [here](https://en.wikipedia.org/wiki/Hangman_(game)). In this problem, the second player will always be the computer, who will be picking a word at random.

### Requirements

Here are the requirements for your game:

1. The computer must select a word at random from the list of available words in `words.txt`.
2. The game must be interactive; the flow of the game should go as follows:
    - At the start of the game, let the user know how many letters the computer's word contains.
    - Ask the user to supply one guess (i.e. letter) per round.
    - The user should receive feedback immediately after each guess about whether their guess appears in the computer's word.
    - After each round, you should also display to the user the partially guessed word so far, as well as letters that the user has not yet guessed.
3. Some additional rules of the game:
    - A user is allowed 8 guesses. Make sure to remind the user of how many guesses s/he has left after each round. Assume that players will only ever submit one character at a time (A-Z).
    - A user loses a guess **only** when s/he guesses incorrectly.
    - If the user guesses the same letter twice, do not take away a guess - instead, print a message letting them know they've already guessed that letter and ask them to try again.
    - The game should end when the user constructs the full word or runs out of guesses. If the player runs out of guesses (s/he "loses"), reveal the word to the user when the game ends.

### Sample Output

<details>
    <summary>The output of a winning game should look like this...</summary>

    Loading word list from file...
    55900 words loaded.
    Welcome to the game, Hangman!
    I am thinking of a word that is 4 letters long.
    -------------
    You have 8 guesses left.
    Available letters: abcdefghijklmnopqrstuvwxyz
    Please guess a letter: a
    Good guess: _ a_ _
    ------------
    You have 8 guesses left.
    Available letters: bcdefghijklmnopqrstuvwxyz
    Please guess a letter: a
    Oops! You've already guessed that letter: _ a_ _
    ------------
    You have 8 guesses left.
    Available letters: bcdefghijklmnopqrstuvwxyz
    Please guess a letter: s
    Oops! That letter is not in my word: _ a_ _
    ------------
    You have 7 guesses left.
    Available letters: bcdefghijklmnopqrtuvwxyz
    Please guess a letter: t
    Good guess: ta_ t
    ------------
    You have 7 guesses left.
    Available letters: bcdefghijklmnopqruvwxyz
    Please guess a letter: r
    Oops! That letter is not in my word: ta_ t
    ------------
    You have 6 guesses left.
    Available letters: bcdefghijklmnopquvwxyz
    Please guess a letter: m
    Oops! That letter is not in my word: ta_ t
    ------------
    You have 5 guesses left.
    Available letters: bcdefghijklnopquvwxyz
    Please guess a letter: c
    Good guess: tact
    ------------
    Congratulations, you won!
</details>

<details>
    <summary>
    And the output of a losing game should look like this...
    </summary>
    
    Loading word list from file...
    55900 words loaded.
    Welcome to the game Hangman!
    I am thinking of a word that is 4 letters long
    -----------
    You have 8 guesses left
    Available Letters: abcdefghijklmnopqrstuvwxyz
    Please guess a letter: a
    Oops! That letter is not in my word: _ _ _ _
    -----------
    You have 7 guesses left
    Available Letters: bcdefghijklmnopqrstuvwxyz
    Please guess a letter: b
    Oops! That letter is not in my word: _ _ _ _
    -----------
    You have 6 guesses left
    Available Letters: cdefghijklmnopqrstuvwxyz
    Please guess a letter: c
    Oops! That letter is not in my word: _ _ _ _
    -----------
    You have 5 guesses left
    Available Letters: defghijklmnopqrstuvwxyz
    Please guess a letter: d
    Oops! That letter is not in my word: _ _ _ _
    -----------
    You have 4 guesses left
    Available Letters: efghijklmnopqrstuvwxyz
    Please guess a letter: e
    Good guess: e_ _ e
    -----------
    You have 4 guesses left
    Available Letters: fghijklmnopqrstuvwxyz
    Please guess a letter: f
    Oops! That letter is not in my word: e_ _ e
    -----------
    You have 3 guesses left
    Available Letters: ghijklmnopqrstuvwxyz
    Please guess a letter: g
    Oops! That letter is not in my word: e_ _ e
    -----------
    You have 2 guesses left
    Available Letters: hijklmnopqrstuvwxyz
    Please guess a letter: h
    Oops! That letter is not in my word: e_ _ e
    -----------
    You have 1 guesses left
    Available Letters: ijklmnopqrstuvwxyz
    Please guess a letter: i
    Oops! That letter is not in my word: e_ _ e
    -----------
    Sorry, you ran out of guesses. The word was else.
</details>

We'll break down the problem into logical sub-tasks, creating helper functions in order for this game to work.


## Is the Word Guessed

We'll start by writing 3 simple functions that will help us easily code the Hangman problem. First, implement the function `is_word_guessed` that takes in two parameters - a string, `secret_word`, and a list of letters, `letters_guessed`. This function returns a boolean - `true` if `secret_word` has been guessed (i.e., all the letters of `secret_word` are in `letters_guessed`) and `false` otherwise.

Example Usage:

```rust
let secret_word = "apple";
let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

let answer = is_word_guessed(secret_word, &letters_guessed);
assert_eq!(answer, false)
```

For this function, you may assume that all the letters in `secret_word` and `letters_guessed` are lowercase.


## Getting the User's Guess

Next, implement the function `get_guessed_word` that takes in two parameters - a string, `secret_word`, and a list of letters, `letters_guessed`. This function returns a string that is comprised of letters and underscores, based on what letters in `letters_guessed` are in `secret_word`. This shouldn't be too different from `is_word_guessed`!

Example Usage:

```rust
let secret_word = "apple";
let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

let result = get_guessed_word(secret_word, &letters_guessed);
assert_eq!(result, "_pp_e".to_string())
```

When inserting underscores into your string, it's a good idea to add at least a space after each one, so it's clear to the user how many unguessed letters are left in the string (compare the readability of `____` with `_ _ _ _` ). This is called *usability* - it's very important, when programming, to consider the usability of your program. If users find your program difficult to understand or operate, they won't use it!

For this problem, you are free to use spacing in any way you wish - our grader will only check that the letters and underscores are in the proper order; it will not look at spacing. We do encourage you to think about usability when designing.

For this function, you may assume that all the letters in `secret_word` and `letters_guessed` are lowercase.


## Printing Out all Available Letters

Next, implement the function `get_available_letters` that takes in one parameter - a list of letters, `letters_guessed`. This function returns a string that is comprised of lowercase English letters - all lowercase English letters that are **not** in `letters_guessed`.

Example Usage:

```rust
let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

let answer = get_available_letters(&letters_guessed);
assert_eq!(answer, "abcdfghjlmnoqtuvwxyz");
```

Note that this function should return the letters in alphabetical order, as in the example above.

For this function, you may assume that all the letters in `letters_guessed` are lowercase.


## The Game

Now you will implement the function `hangman`, which takes one parameter—the `secretWord` the user is to guess. This starts up an interactive game of Hangman between the user and the computer. Be sure you take advantage of the three helper functions, `isWordGuessed`, `getGuessedWord`, and `getAvailableLetters`, that you've defined in the previous part.

### Hints:

- You should start by noticing where we're using the provided functions (at the top of `ps3_hangman.py`) to load the words and pick a random one. Note that the functions `loadWords` and `chooseWord` should only be used on your local machine, not in the tutor. When you enter in your solution in the tutor, you only need to give your `hangman` function.
- Consider using `lower()` to convert user input to lower case. For example:
    
    ```python
    guess = 'A'
    guessInLowerCase = guess.lower()
    ```
    
- Consider writing additional helper functions if you need them!
- There are four important pieces of information you may wish to store:
    1. `secretWord`: The word to guess.
    2. `lettersGuessed`: The letters that have been guessed so far.
    3. `mistakesMade`: The number of incorrect guesses made so far.
    4. `availableLetters`: The letters that may still be guessed. Every time a player guesses a letter, the guessed letter must be removed from `availableLetters` (and if they guess a letter that is not in `availableLetters`, you should print a message telling them they've already guessed that - so try again!).