pub struct Process {
    time_in_queue: i32,
    task_time: i32,
    time_processed: i32,
}

impl Process {
    pub fn new(task_time: i32) -> Process {
        Process {
            time_in_queue: 0,
            task_time,
            time_processed: 0,
        }
    }

    pub fn get_task_time(&self) -> i32 {
        self.task_time
    }

    pub fn add_time_in_queue(&mut self) {
        self.time_in_queue += 1;
    }

    pub fn get_time_in_queue(&self) -> i32 {
        self.time_in_queue
    }

    pub fn add_time_processed(&mut self) {
        self.time_processed += 1;
    }

    pub fn get_time_processed(&self) -> i32 {
        self.time_processed
    }
}
