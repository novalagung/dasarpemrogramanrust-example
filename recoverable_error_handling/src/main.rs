use std::io;
use std::io::Write;

fn print_welcome_message() {
    println!("Welcome to file maker app!");
    println!();

    println!("Available command:");
    println!("1. Print files");
    println!("2. Read file");
    println!("3. Delete file");
    println!("9. Exit");
}

enum Command {
    PrintFiles,
    ReadFile,
    DeleteFile,
    ExitProgram,
}

fn stdout_flush() -> Result<(), String> {
    match io::stdout().flush() {
        Err(err) => Err(err.to_string()),
        Ok(()) => Ok(())
    }
}

fn read_entry() -> Result<String, String> {
    let mut message = String::new();
    let reader_res = io::stdin().read_line(&mut message);

    let content = match reader_res {
        Ok(_) => message.trim().to_string(),
        Err(err) => {
            return Err(err.to_string())
        }
    };

    return Ok(content)
}

fn validate_command(cmd: &str) -> Result<Command, &'static str> {
    match cmd {
        "1" => Ok(Command::PrintFiles),
        "2" => Ok(Command::ReadFile),
        "3" => Ok(Command::DeleteFile),
        "9" => Ok(Command::ExitProgram),
        _ => Err("unrecognized command")
    }
}

fn run_program() -> Result<(), String> {
    loop {
        println!();
        print!("Enter your command: ");

        // error handling using operator ?
        let _ = stdout_flush()?;

        // error handling using guard method
        let user_entry = match read_entry() {
            Err(err) => {
                println!("ERROR. unable to continue the program. {}", err);
                continue;
            },
            Ok(txt) => txt,
        };

        // error handling using common match implementation
        let cmd_result = validate_command(&user_entry);
        match cmd_result {
            Err(err) => {
                println!("ERROR. {}", err);
                continue;
            },
            _ => {}
        };
        let cmd = cmd_result.unwrap();

        // check command
        match cmd {
            Command::PrintFiles => {
                println!("print files ...")
            },
            Command::ReadFile => {
                println!("read file ...")
            },
            Command::DeleteFile => {
                println!("delete file ...")
            },
            Command::ExitProgram => {
                println!("Exiting program ...");
                return Ok(());
            },
        }
    }
}

fn main() {
    print_welcome_message();
    let _ = run_program();
}
