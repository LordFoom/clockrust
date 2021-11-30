use color_eyre::{ eyre::eyre,Result };

pub struct CommandRunner{
    conn_string: String,
}

impl CommandRunner{
    pub fn new(conn_string:String)->CommandRunner{
        CommandRunner{
            conn_string
        }
    }

    pub fn run_command(self, check_str: &str)->Result<String>{
        //is it one of our commands, if so return a positive result
        return if check_str.starts_with("clock-in") {
            //break command into at least 2, possibly 3 parts
            let cic: Vec<&str> = check_str.split(' ').collect();
            if cic.len() < 2 || cic.len() > 3 {
                return Err(eyre!("FAIL, USAGE: clock-in HASH optional_notes"))
            }
            //hash[0] will be  "clock-in"
            let hash = cic[1];
            let notes = if cic.len() == 3 {
                cic[2]
            } else {
                ""
            };
            //insert into db

            Ok("Success clock-in TODO PUTIN HASH".to_string())
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
}

#[cfg(test)]
mod tests{
   use super::*;

    ///we try to do the run a command that doesn't exist
    #[test]
    fn test_bad_command(){
        let cmd_runner = CommandRunner::new("./test.db".to_string());
        let result = cmd_runner.run_command("badcommand");
        assert_eq!(result.unwrap_err(),"FAIL, supported commands: clock-in, clock-out".to_string());
    }

}