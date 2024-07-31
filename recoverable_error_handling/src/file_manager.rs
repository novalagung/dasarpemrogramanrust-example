use std::path::Path;
use std::fs;

pub fn print_files() -> Result<(), String> {
    let path = Path::new("./files");

    // if files folder not exists, create it
    if !path.is_dir() {

        // error handling using match
        match fs::create_dir(path) {
            Err(err) => {
                return Err(err.to_string());
            },
            Ok(_) => {},
        }
    }

    // error handling using guard method
    let dir = match fs::read_dir(path) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(entry) => entry,
    };

    let mut count = 0;
    for file in dir {
        count = count + 1;

        // error handling using match
        match file {
            Err(err) => {
                return Err(err.to_string());
            },
            Ok(entry) => {
                println!(" -> {:?}", entry.path());
            }
        }
    };

    if count == 0 {
        print!("No files");
    }

    Ok(())
}

pub fn create_file() -> Result<(), String> {
    let path = Path::new("./files");

    print!("Enter filename: ");
    
    // error handling using operator ?
    let _ = crate::file_utility::stdout_flush()?;

    // error handling using guard method
    let filename = match crate::file_utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(txt) => txt,
    };

    print!("Enter file content: ");
    
    // error handling using operator ?
    let _ = crate::file_utility::stdout_flush()?;

    // error handling using guard method
    let content = match crate::file_utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(txt) => txt,
    };

    // error handling using match
    match fs::write(path.join(filename), content) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(()) => {},
    }
    
    Ok(())
}

pub fn read_file() -> Result<(), String> {
    let path = Path::new("./files");

    print!("Enter filename: ");
    
    // error handling using operator ?
    let _ = crate::file_utility::stdout_flush()?;

    // error handling using guard method
    let filename = match crate::file_utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(txt) => txt,
    };

    // error handling using guard method
    let content = match fs::read_to_string(path.join(filename)) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(txt) => txt,
    };

    println!("File content: {:?}", content);

    Ok(())
}

pub fn delete_file() -> Result<(), String> {
    let path = Path::new("./files");

    print!("Enter filename: ");
    
    // error handling using operator ?
    let _ = crate::file_utility::stdout_flush()?;

    // error handling using guard method
    let filename = match crate::file_utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(txt) => txt,
    };

    // error handling using match
    match fs::remove_file(path.join(filename)) {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok(_) => {
            println!("File deleted");
        },
    };

    Ok(())
}
