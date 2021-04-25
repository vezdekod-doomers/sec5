use crate::sender::Sender;
use std::error::Error;

pub struct PrintSender {}

impl Sender for PrintSender {
    fn init(&mut self) {

    }

    fn send(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        println!("MSG: {}", msg);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "print"
    }
}