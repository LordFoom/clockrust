use color_eyre::{eyre::eyre, Report, Result};
use crate::db::{ClockRust, ClockRuster};
use std::fmt::{Display, Formatter};

pub trait Command{
    // fn new(connection_str: &str)->Self;
    fn run_command(&self)->Result<(), Report>;
}
pub struct ClockIn<'a>{
    task: &'a str
}

pub struct ClockOut<'a>{
    task: &'a str
}

impl<'a> Command for ClockIn<'a>{
    fn run_command(&self) -> Result<(), Report> {
        Ok(())
    }
}

impl<'a> ClockIn<'a>{
    fn new(activity: &str) ->ClockIn{
        ClockIn{
            task: activity
        }
    }
}

impl Display for ClockIn{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        write!(f, "clock-in {}", task)

    }
}

impl<'a> Command for ClockOut<'a>{

    fn run_command(&self) -> Result<(), Report> {
        Ok(())
    }
}

impl<'a> ClockOut<'a>{
    fn new(activity: &str)->ClockOut{
        ClockOut{
            task: activity
        }
    }
}

impl Display for ClockOut{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "clock-out {}", task)
    }
}
// impl CommandConstructor {

    pub fn create_command(check_str: &str) ->Result<Box<dyn Command + '_>, Report>{

        //is it one of our commands, if so return a positive result
        return if check_str.starts_with("clock-in") {
            //break command into at least 2, possibly 3 parts
            let cic: Vec<&str> = check_str.split(' ').collect();
            if cic.len() < 2 || cic.len() > 2 {
                return Err(eyre!("FAIL, USAGE: clock-in HASH optional_notes"))
            }
            //hash[0] will be  "clock-in"
            let clock_string = cic[1];
            let notes = if cic.len() == 3 {
                cic[2]
            } else {
                ""
            };
            let ci = Box::new(ClockIn::new(clock_string));
            Ok(ci)
        } else if check_str.starts_with("clock-out") {
            //insert into db
            let cic: Vec<&str> = check_str.split(' ').collect();
            if cic.len() < 2 || cic.len() > 2 {
                return Err(eyre!("FAIL, USAGE: clock-out HASH optional_notes"))
            }
            Err(eyre!("FAIL: not yet implemented clock-out"))
        } else {
            Err(eyre!("FAIL, supported commands: clock-in, clock-out"))
        }
    }
// }

#[cfg(test)]
mod tests{
    use color_eyre::Report;
    use super::*;

    ///we try to do the run a command that doesn't exist
    #[test]
    fn test_bad_command(){
        // let cmd_runner = CommandConstructor::new("./test.db".to_string());
        let result = create_command("badcommand");
        let report = result.err().unwrap();
        assert_eq!(report.to_string(), "FAIL, supported commands: clock-in, clock-out".to_string());
    }

    #[test]
    fn test_clock_in(){

    }

}