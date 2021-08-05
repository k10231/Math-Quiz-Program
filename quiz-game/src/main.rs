use rand::Rng;
use std::io;

fn main() {
    //getting user input -> no of questions
    let mut no_of_questions = String::new();
    io::stdin()
        .read_line(&mut no_of_questions)
        .expect("failed to read from stdin");

    match no_of_questions.trim().parse::<u32>() {
        Ok(int_input) => {
            // loop for no of times questions

            println!("Number of questions : {}", int_input);
            for i in 0..int_input {
                // println!("{}", i);
                println!("{}.) Question", i + 1);
                create_question()
            }
        }
        Err(..) => println!("Please provide valid number"),
    };
}

//generate 2 random numbers, 1 operator, take input, check for answers
fn create_question() {
    let operators = ["+", "-", "/", "*"];

    let first_num: i32 = rand::thread_rng().gen_range(1..101);
    let second_num: i32 = rand::thread_rng().gen_range(1..101);
    let choose_operator: usize = rand::thread_rng().gen_range(0..operators.len());
    let operator_choosen = &operators[choose_operator];
    println!(
        "{} {} {} = ?",
        first_num, &operators[choose_operator], second_num
    );
    let mut answer = String::new();
    let correct_answer = do_calculation(&operator_choosen, first_num, second_num);
    println!(
        "a.) {:?} \nb.){} \nc.){} \nd.){}",
        &correct_answer,
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101)
    );

    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read from stdin");

    if &answer.trim().parse::<i32>().unwrap() == &correct_answer {
        println!("you're correct")
    } else {
        println!("you're wrong")
    }
}

// get right answer ¯\_(ツ)_/¯
fn do_calculation(operator_choosen: &str, first_num: i32, second_num: i32) -> i32 {
    match operator_choosen {
        "+" => first_num + second_num,
        "-" => first_num - second_num,
        "/" => first_num / second_num,
        "*" => first_num * second_num,
        _ => 0,
    }
}
