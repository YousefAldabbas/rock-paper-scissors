use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use rand::Rng;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

enum Result {
    Win,
    Loose,
    Draw,
}

fn print_in_square(text: String) {
    let width = text.len() + 2;

    let horizontal_line = "─".repeat(width);
    let vertical_line = "│";

    println!("┌{}┐", horizontal_line);
    println!("{} {} {}", vertical_line, text, vertical_line);
    println!("└{}┘", horizontal_line);
}

fn get_winner(u: Choice, c: &Choice) -> Result {
    let result = match (u, c) {
        (Choice::Rock, Choice::Rock) => "Draw",
        (Choice::Rock, Choice::Paper) => "Computer wins",
        (Choice::Rock, Choice::Scissor) => "User wins",
        (Choice::Paper, Choice::Rock) => "User wins",
        (Choice::Paper, Choice::Paper) => "Draw",
        (Choice::Paper, Choice::Scissor) => "Computer wins",
        (Choice::Scissor, Choice::Rock) => "Computer wins",
        (Choice::Scissor, Choice::Paper) => "User wins",
        (Choice::Scissor, Choice::Scissor) => "Draw",
    };
    println!("*******{}*******", &result);
    let score = match result {
        "User wins" => Result::Win,
        "Computer wins" => Result::Loose,
        _ => Result::Draw,
    };
    score
}

fn get_random_choice(choices: &Vec<Choice>) -> &Choice {
    let rnum = rand::thread_rng().gen_range(0..choices.len());
    &choices[rnum]
}

fn get_user_input(items: Vec<&str>) -> &str {
    let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    match selection {
        Some(index) => items[index],
        None => panic!("User did not select anything"),
    }
}

fn get_user_choice(user_input: &str) -> Choice {
    let user_choice = match user_input {
        "rock" => Choice::Rock,
        "paper" => Choice::Paper,
        "scissor" => Choice::Scissor,
        _ => panic!("invalid input"),
    };
    user_choice
}
fn main() {
    println!("Do you want to play a game?");
    let mut u_score = 0;
    let mut c_score = 0;
    let yes_no = vec!["Yes", "No"];
    loop {
        let user_input = get_user_input(yes_no.clone());
        println!("{}------------------",user_input);
        match user_input {
            "Yes" => play(&mut u_score, &mut c_score),
            _ => {
                println!("Ok Goodbye");
                break;
            }
        }
        println!("Your score : {}\ncomputer score : {}", u_score, c_score);
        println!("Do you want to play again ?");
    }
}

fn play(u_score: &mut i32, c_score: &mut i32) {
    println!("THE GAME JUST STARTED");
    let choices = vec![Choice::Rock, Choice::Scissor, Choice::Paper];
    println!("Choose your weapon: [rock, paper, scissor]");
    let items = vec!["rock", "paper", "scissor"];
    let user_choice = get_user_choice(get_user_input(items));
    let computer_choice = get_random_choice(&choices);

    print_in_square(format!(
        "user choice ==> {:#?} || computer choice ==> {:#?}",
        user_choice, computer_choice
    ));
    let result = get_winner(user_choice, computer_choice);
    match result {
        Result::Win => *u_score += 1,
        Result::Loose => *c_score += 1,
        Result::Draw => {}
    }
}
