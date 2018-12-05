pub trait Notifyer {
    fn notify_success(&self) -> Result<(), String>;
    fn notify_failure(&self) -> Result<(), String>;
}

pub struct TermOut;

impl Notifyer for TermOut {
    fn notify_success(&self) -> Result<(), String>{
        println!("Event Creation Successful!");
        Ok(())
    }

    fn notify_failure(&self) -> Result<(), String> {
        println!("Event Creation unsuccessful!");
        Ok(())
    }
}