use std::fmt::{Display, Formatter};

use color_eyre::{eyre::eyre, Report, Result};

enum CommandType {
    ClockIn,
    ClockOut,
}

impl Display for CommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandType::ClockIn => write!(f, "clock-in"),
            CommandType::ClockOut => write!(f, "clock-out"),
        }
    }
}

// pub trait Command{
//     // fn new(connection_str: &str)->Self;
//     fn run_command(&self)->Result<(), Report>;
// }
pub struct Command {
    cmd: CommandType,
    task:  String,
}


impl Command {
    fn new(cmd: CommandType, task: String) -> Self {
        Self {
            cmd,
            task,
        }
    }

    pub fn run_command(&self)-> Result<(), Report>{
        // match self.cmd{
        //     Cl
        // }
        Ok(())
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.cmd, self.task)
    }
}


pub fn create_command(check_str: &str) -> Result<Command, Report> {

    // let task = split.as_str();
    let parts:Vec<&str> = check_str.split("::").collect();
    let cmd = match parts[0]   {
        "clock-in" => CommandType::ClockIn ,
        "clock-out" =>  CommandType::ClockOut ,

        //unsupported command
        _ => return Err(eyre!("Fail, available commands: clock-in | clock-out"))
    };

    if(parts.len()!=3){
        return Err(eyre!("FAIL, usage command::time::title"))
    }
    let time_str = parts[1];
    let title_str  = parts[2];

    //is it one of our commands, if so return a positive result
    return if check_str.starts_with("clock-in") {
        //break command into at least 2, possibly 3 parts
        // let mut split = check_str.split(' ');
        // split.next();

        ///command string is the first part, relevant time is the second part, task key (title) is the third part
        let task = parts[1..].join(" ");
        if task.is_empty() {
            Err(eyre!("FAIL, usage: clock-in task that can be many words"))
        }else{
            let ci = Command::new(CommandType::ClockIn, task);
            Ok(ci)
        }
    } else if check_str.starts_with("clock-out") {
        //insert into db
        let parts:Vec<&str> = check_str.split(' ').collect();
        let task = parts[1..].join(" ");
        if  task.is_empty() {
            Err(eyre!("FAIL, usage: clock-out task that can be many words"))
        }else {
            let co = Command::new(CommandType::ClockOut, task);
            Ok(co)
        }
    } else {
        Err(eyre!("FAIL, supported commands: clock-in, clock-out"))
    };
}
// }

#[cfg(test)]
mod tests {
    use color_eyre::Report;

    use super::*;

    ///we try to do the run a command that doesn't exist
    #[test]
    fn test_bad_command() {
        // let cmd_runner = CommandConstructor::new("./test.db".to_string());
        let result = create_command("badcommand");
        let report = result.err().unwrap();
        assert_eq!(report.to_string(), "FAIL, supported commands: clock-in, clock-out".to_string());
    }

    #[test]
    fn test_clock_in() {
        let result = create_command("clock-in this is a test");
        match result{
            Ok(clock_in) => assert_eq!(clock_in.to_string(), "clock-in this is a test"),
            Err(why) => {
                println!("We have FAILED: {}", why);
                assert_eq!(false , true);//let it end
            }
        }
    }

    #[test]
    fn test_clock_out(){
        let result = create_command("clock-out this is the clock out test");
        match result{
            Ok(clock_out) => assert_eq!(clock_out.to_string(), "clock-out this is the clock out test"),
            Err(why) => {
                println!("We have FAILED: {}", why);
                assert_eq!(false, true);//let it end
            }
        }
    }
}