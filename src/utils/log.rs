use std::io::{self, Error, Write};

pub struct Log {}
impl Log {
    pub fn info(&self, text: &str) -> Result<(), Error> {
        io::stdout().write(text.as_bytes())?;
        io::stdout().write("\n".as_bytes())?;
        io::stdout().flush()?;
        return Ok(());
    }

    pub fn warning(&self, text: &str) -> Result<(), Error> {
        return self.info(&format!("Warning: {}", text));
    }
}
