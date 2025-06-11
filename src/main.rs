use std::io::{Write, stdin, stdout};
fn main() {
    let mut user_input: String = String::default();
    let word: &str = "apple";
    let mut qwinc: usize = 0;
    let mut user_space: usize = 0;
    let mut answer_array: [usize; 6] = [0, 0, 0, 0, 0, 0]; // 1 green 2, yellow, 3 grey
    
    // validate if check to make user_input allowed
    for qua in 0..6 {
        user_input.clear();
        print!("Guess word guess number {qua}: ");
        stdout().flush().expect("aa");
        stdin().read_line(&mut user_input).expect("iiii know");
        println!("Your input: {user_input}");
        while user_input.trim().chars().count() != 5 {
            user_input.clear();
            println!("Error: Please enter a 5 character word");
            print!("Guess word guess number {qua}: ");
            stdout().flush().expect("aa");
            stdin().read_line(&mut user_input).expect("iiii know");
            println!("Your input: {user_input}");
        }
        qwinc = wmatch(&user_input, &word, answer_array);
        println!("qwinc: {qwinc}");

        if qwinc == 5 {
            println!("You won! Word is: {word}");
            break;
        }
    }
    if qwinc != 5 {
        println!("You lost! Word is: {word}");
    }
}
fn wmatch<'a>(user_input: &'a String, word: &'a str, mut answer_array: [usize; 6]) -> usize {
    let mut increment_space: usize = 0;
    let mut yarray_pos: usize = 0;
    let mut qwinc: usize = 0;
    let mut yelcheck: usize = 0;
    let mut user_space: usize = 0;
    let mut yellow_array: [usize; 5] = [5, 5, 5, 5, 5];
    let mut green_array: [usize; 5] = [5, 5, 5, 5, 5];

    if user_space == 5 {
        println!("You lost! Word is: {word}");
    }
    while user_space < 5 {
        if user_input.trim().chars().nth(user_space) == word.chars().nth(user_space) {
            green_array[user_space] = 9;
            increment_space = user_space + 1;
            //  println!("you have a green at spot {increment_space}"); // not needed
        } else {
        }
        user_space = user_space + 1;
    }
    user_space = 0;
    while user_space < 5 {
        let mut compare_sapce: usize = 0;
        if user_input.trim().chars().nth(user_space) == word.chars().nth(user_space) {
            increment_space = user_space + 1;
            //println!("you have a green at spot {increment_space}");
            answer_array[user_space] = 1;
        } else {
            compare_sapce = 0;
            while compare_sapce < 5 {
                let mut found_yellow: bool = false;
                increment_space = user_space + 1;
                compare_sapce = compare_sapce + 1;
                let mut x: usize = 0;
                while x < 5 {
                    if green_array[x] == 9 {
                        //println!("!!!!green found at yellow array pos, {x}"); //debug line
                        found_yellow = true;
                        break;
                    } else {
                        // println!("x = {x}"); // debug line
                        x = x + 1
                    }
                }
                if user_input.trim().chars().nth(user_space) == word.chars().nth(compare_sapce) {
                    yelcheck = 0;
                    while yelcheck < 5 {
                        if compare_sapce == yellow_array[yelcheck] {
                            found_yellow = true;
                            break;
                        } else {
                            yelcheck = yelcheck + 1;
                        }
                    }
                    if found_yellow {
                        continue;
                    }
                    yellow_array[yarray_pos] = compare_sapce;
                    yarray_pos = yarray_pos + 1;
                    //println!("You found a yellow at spot {increment_space}");
                    answer_array[user_space] = 2;
                    compare_sapce = 6;
                }
            }
            if compare_sapce != 6 {
                //println!("You found a gray at spot {increment_space}");
                answer_array[user_space] = 3;
            }
        }
        user_space = user_space + 1;
        // println!("i = {user_space}");

        if user_space == 5 {
            for b in 0..5 {
                println!("Result: {}", answer_array[b]);
                if answer_array[b] == 1 {
                    qwinc = qwinc + 1
                }
            }
        }
    }
    return qwinc;
}