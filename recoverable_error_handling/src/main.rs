mod file_action_constant;
mod file_manager;
mod file_utility;

fn run_program() -> Result<(), String> {
    println!("Welcome to file maker app!");

    loop {
        println!();
        println!("Available command:");
        println!("1. Print files");
        println!("2. Create file");
        println!("3. Read file");
        println!("4. Delete file");
        println!("9. Exit");

        println!();
        print!("Enter your command: ");

        // error handling using operator ?
        let _ = file_utility::stdout_flush()?;

        // error handling using guard method
        let user_entry = match file_utility::read_entry() {
            Err(err) => {
                println!("ERROR. unable to continue the program. {}", err);
                continue;
            },
            Ok(txt) => txt,
        };

        // error handling using match
        let cmd_result = file_action_constant::validate_command(&user_entry);
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
            file_action_constant::Command::PrintFiles => {
                // error handling using operator ?
                file_manager::print_files()?;
            },
            file_action_constant::Command::CreateFile => {
                // error handling using operator ?
                file_manager::create_file()?;
            },
            file_action_constant::Command::ReadFile => {
                // error handling using operator ?
                file_manager::read_file()?;
            },
            file_action_constant::Command::DeleteFile => {
                // error handling using operator ?
                file_manager::delete_file()?;
            },
            file_action_constant::Command::ExitProgram => {
                println!("Exiting program ...");
                return Ok(());
            },
        }
    }
}

fn main() {
    let _ = run_program();
}
