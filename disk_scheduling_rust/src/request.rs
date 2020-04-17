pub struct Request {
    block_num: i32,
    is_realtime: bool,
    time_in_queue: i32,
    time_to_handle: i32,
}

impl Request {
    pub fn new(
        block_num: i32,
        is_realtime: bool,
        time_in_queue: i32,
        time_to_handle: i32,
    ) -> Request {
        Request {
            block_num,
            is_realtime,
            time_in_queue,
            time_to_handle,
        }
    }

    pub fn get_block_num(&self) -> i32 {
        self.block_num
    }

    pub fn get_time_in_queue(&self) -> i32 {
        self.time_in_queue
    }

    pub fn is_realtime(&self) -> bool {
        self.is_realtime
    }

    pub fn distance_to_head(&self, head_pos: i32) -> i32 {
        if head_pos > self.block_num {
            head_pos - self.block_num
        } else {
            self.block_num - head_pos
        }
    }

    pub fn add_time_in_queue(&mut self) {
        self.time_in_queue += 1;
    }

    pub fn time_remaining(&self) -> i32 {
        self.time_to_handle - self.time_in_queue
    }
}
