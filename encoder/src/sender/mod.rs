use std::error::Error;

mod email;
mod print;

pub use print::PrintSender;
pub use email::EmailSender;

pub trait Sender {
    fn init(&mut self);

    fn send(&self, msg: &str) -> Result<(), Box<dyn Error>>;

    fn name(&self) -> &'static str;
}