use std::io;
use std::io::Write;

pub fn read_entry() -> Result<String, String> {
    let mut message = String::new();
    let reader_res = io::stdin().read_line(&mut message);

    // error handling using guard method
    let content = match reader_res {
        Ok(_) => message.trim().to_string(),
        Err(err) => {
            return Err(err.to_string())
        }
    };

    return Ok(content)
}

pub fn stdout_flush() -> Result<(), String> {

    // error handling using basic implementation of keyword match
    match io::stdout().flush() {
        Err(err) => Err(err.to_string()),
        Ok(()) => Ok(())
    }
}
