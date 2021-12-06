// use rusqlite::{Connection, params};

///Using a trait to help with testing....
pub trait ClockRuster{
    fn clock_in(self, task_str: &str);
    fn clock_out(self, task_str: &str);

}
#[derive(Default)]
pub struct ClockRust;

impl ClockRuster for ClockRust{
    fn clock_in(self, task_str: &str) {
        todo!()
    }

    fn clock_out(self, task_str: &str) {
        todo!()
    }
}
