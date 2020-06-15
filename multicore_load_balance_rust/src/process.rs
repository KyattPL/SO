use rand::*;

const MIN_TIME: i32 = 750;
const MAX_TIME: i32 = 1250;

pub struct Process {
    pub required_power: i32,
    pub time_left: i32,
}

impl Process {
    pub fn new() -> Process {
        let mut rng = thread_rng();
        let pow = rng.gen_range(1, 10);
        let time = rng.gen_range(MIN_TIME, MAX_TIME);
        Process {
            required_power: pow,
            time_left: time,
        }
    }
}
