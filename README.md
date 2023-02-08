# wordle-cli-rs

## General

This is an example recreation of the game Wordle, for the commandline, written in Rust.

At the time of writing (08/02/2023 in dd/mm/yy format) I am new to Rust, and I do not claim the code is optimal. There are probably lots of improvements to be made.

A random word loaded from the words file is selected and the player has a total of 6 guesses, just like the original. With each guess every letter is evaluated and colored according to this table:
|**Color**|Meaning
---|---
|**Green**|Correct Letter and Correct Place
|**Yellow**|Correct Letter but Wrong Place
|**Red**|Wrong Letter and Wrong Place

The words list can be modified by changing the words file.

## Installation & Running
Just run `cargo build --release && cargo run --release`.

## Known Bugs & Issues
As the code currently colors a character yellow if that character exists in the randomly selected word, in a guess with one single letter used multiple times while the letter exists only once in the random word, the remaining letters would color yellow even though they don't exist. I might address this issue later.