use std::env;
use std::fs;
use std::process;
use std::process::Command;
use std::str;

struct UserInput {
    file_name: String,
    should_contain: bool,
    command_being_tested: String,
}

fn parse_args(args: Vec<String>) -> Option<UserInput> {
    if args.len() == 5 && args[2] == "to" && args[3] == "output" {
        Some(UserInput {
            should_contain: true,
            command_being_tested: args[1].clone(),
            file_name: args[4].clone(),
        })
    } else if args.len() == 6 && args[2] == "to" && args[3] == "not" && args[4] == "output" {
        Some(UserInput {
            should_contain: false,
            command_being_tested: args[1].clone(),
            file_name: args[5].clone(),
        })
    } else {
        None
    }
}

fn run_command(cmd: &str) -> String {
    let outout = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    }
    .stdout;

    String::from_utf8(outout).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_result = parse_args(args);
    if arg_result.is_none() {
        println!("Usage:");
        println!("expect \"command\" to output hello.txt");
        println!("expect \"command\" to not output hello.txt");
        process::exit(1);
    }

    let user_input = arg_result.unwrap();

    let file_read = fs::read_to_string(&user_input.file_name);

    if !file_read.is_ok() {
        println!("Can't read from {}", &user_input.file_name);
        return;
    }

    let file_content = file_read.unwrap().replace("\r\n", "\n");
    let output_to_test = run_command(&user_input.command_being_tested).replace("\r\n", "\n");

    if output_to_test.contains(&file_content) == user_input.should_contain {
        println!("Passed. Output matched the expectation.");
        process::exit(0);
    } else {
        println!("Failed. Output did not match the expectation.");
        process::exit(1);
    }
}
