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
}
