pub struct Process {
    time_in_queue: i32,
    time_to_execute: i32,
}

impl Process {
    pub fn new(time_to_execute: i32) -> Process {
        Process {
            time_in_queue: 0,
            time_to_execute
        }
    }
}