use std::env;
use std::fs;
use std::process;
use std::process::Command;
use std::str;

#[derive(PartialEq, Eq)]
enum ComparisonType {
    ShouldContain,
    ShouldNotContain,
    ShouldEqual,
}

struct UserInput {
    file_name: String,
    comparison: ComparisonType,
    command_being_tested: String,
}

fn parse_args(args: Vec<String>) -> Option<UserInput> {
    if args.len() == 5 && args[2] == "to" && args[3] == "output" {
        Some(UserInput {
            comparison: ComparisonType::ShouldContain,
            command_being_tested: args[1].clone(),
            file_name: args[4].clone(),
        })
    } else if args.len() == 6 && args[2] == "to" && args[3] == "not" && args[4] == "output" {
        Some(UserInput {
            comparison: ComparisonType::ShouldNotContain,
            command_being_tested: args[1].clone(),
            file_name: args[5].clone(),
        })
    } else if args.len() == 6 && args[2] == "to" && args[3] == "output" && args[4] == "exactly" {
        Some(UserInput {
            comparison: ComparisonType::ShouldEqual,
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
        println!("expect \"command\" to output sample.txt");
        println!("expect \"command\" to not output sample.txt");
        println!("expect \"command\" to output exactly sample.txt");
        process::exit(1);
    }

    let user_input = arg_result.unwrap();

    let file_read = fs::read_to_string(&user_input.file_name);

    if !file_read.is_ok() {
        println!("Can't read from {}", &user_input.file_name);
        process::exit(1);
    }

    let file_content = file_read.unwrap().replace("\r\n", "\n");
    let output_to_test = run_command(&user_input.command_being_tested).replace("\r\n", "\n");

    if (output_to_test.contains(&file_content)
        && user_input.comparison == ComparisonType::ShouldContain)
        || (output_to_test == file_content && user_input.comparison == ComparisonType::ShouldEqual)
    {
        println!("Passed. Output matched the expectation.");
        process::exit(0);
    } else {
        println!("Failed. Output did not match the expectation.");
        process::exit(2);
    }
}
