use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout, Stdout};

#[derive(PartialEq, Debug)]

pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut Stdout: std::io::Stdout = stdout();

        //Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        //print agent statement
        Stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        //make selected color
        Stdout.execute(SetForegroundColor(statement_color)).unwrap();
        print!("Agent: {}: ", agent_statement);

        //reset color
        Stdout.execute(ResetColor).unwrap();
    }
}

//get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    //print a question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    //reset color
    stdout.execute(ResetColor).unwrap();

    //Read user input

    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("failed to read response !");

    //trim whitespace and return
    return user_response.trim().to_string();
}

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING ! You are about to run code written entirely by AI. ");
        println!("it is recommended to review the code first and continue. ");

        stdout.execute(ResetColor).unwrap();

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] all good");


        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] let's stop this project");

        stdout.execute(ResetColor).unwrap();


        let mut human_response: String = String::new();

        stdin().read_line(&mut human_response).expect("Failed to read response.");


        let human_response: String = human_response.trim().to_lowercase();


        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("invalid input. please select '1' or '2'")
            }
        }



    }   
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall
            .print_agent_message(
                "Managing Agent", 
                "Testing testing, processing something");
    }
}