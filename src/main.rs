use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use Vec;
use rand::Rng;
use colored::Colorize;

fn main() {

    let mut words: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./words") {
        for line in lines {
            if let Ok(word) = line {
                words.push(word);
            }
        }
    }

    let randnum = rand::thread_rng().gen_range(0..(&words).len());
    let randword: String;
    if let Some(rword) = words.get(randnum) {
        randword = rword.to_string();
    }
    else {
        randword = String::from("goose");
    }

    //println!("randword is {randword}");

   
    let randwordletters: Vec<char> = randword.chars().collect();


    println!("\n\nwordle-rs-cli\n\nYou have a total of 6 tries.\nGood luck.\n");
    let mut tries = 6;
    while tries > 0 {

        // Clearing the screen also sweeps away previous guesses  
        //print!("{}[2J", 27 as char); // clears the screen by sending a control character according to this page: https://rosettacode.org/wiki/Terminal_control/Clear_the_screen

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // this is unsafe
        
        if guess.len() == 6 { // word length + \0
            
            let guessletters: Vec<char> = guess.chars().collect();

            let mut correct: bool = true;

            println!("\n- - - - - ");

            if words.contains(&(guess[0..5]).to_string()) {
             

            for i in 0..5 {
                if guessletters.get(i) != randwordletters.get(i) {
                    
                    if let Some(ithletter) = guessletters.get(i) {

                    
                    if randwordletters.contains(ithletter) {
                    
                    correct = false;
                    let mut tempvector = randwordletters.clone();
                    
                    // I have to write a simple find loop here as I am not aware of any find function in Rust's Vector implementation
                    for i in 0..tempvector.len() {
                        if Some(ithletter) == tempvector.get(i) {
                            tempvector.remove(i);
                            break;
                        }
                    }
                    if tempvector.contains(ithletter) {
                        print!("{} ", ithletter.to_string().yellow());
                    }
                    else {
                        print!("{} ", ithletter.to_string().red());
                    }
                    continue;
                    }
                    print!("{} ", ithletter.to_string().red());
                    correct = false;
                    }
                }
                else {
                    if let Some(ithletter) = guessletters.get(i) {

                        print!("{} ", ithletter.to_string().green())
                    }
                }
            }

            println!("\n- - - - - \n");
            

            if correct {
                println!("You guessed the word correctly!");
                break;
            }
            else {
                tries -= 1;
                //println!("Incorrect! It's Danke Schön, verdammt! Say it! You have {tries} tries left."); // the original line, from Wolfenstein
                println!("Your guess was incorrect. {tries} tries left.");

            }
               
        }
        else {
            println!("Word not in word list.")
        }

        }
        else {
            println!("You need to input a word with 5 letters.");
        }
    }
    println!("\nThe word was {randword}!\n");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
