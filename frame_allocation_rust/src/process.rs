use rand::prelude::*;

pub struct Process {
    page_min: i32,
    page_max: i32,
    pub requests: Vec<i32>,
    pub frames: Vec<i32>,
    first_frame: i32,
    last_frame: i32,
    pub time_window: i32,
    pub page_faults: i32,
    pub is_stopped: bool,
}

impl Process {
    pub fn new(page_min: i32, page_max: i32) -> Process {
        Process {
            page_min,
            page_max,
            requests: vec![],
            frames: vec![],
            first_frame: 0,
            last_frame: 0,
            time_window: 0,
            page_faults: 0,
            is_stopped: false,
        }
    }

    pub fn generate_requests(&mut self) {
        let mut rng = rand::thread_rng();
        let requests_no = rng.gen_range(1000, 10000);
        let locality_increment = rng.gen_range(1, 6);
        let local_chance = rng.gen_range(20, 40);
        let length_of_local_max = rng.gen_range(5, 100);

        let mut chance = local_chance;
        let mut current_request = 1;
        let mut length_of_local = 0;
        let mut lower_boundary = 0;
        let mut upper_boundary = 0;

        while current_request <= requests_no {
            if rng.gen_range(0, 100) <= chance && length_of_local == 0 {
                length_of_local = rng.gen_range(4, length_of_local_max);
                chance = local_chance;
                lower_boundary =
                    rng.gen_range(self.page_min, (self.page_min + self.page_max) / 2 + 1);
                upper_boundary =
                    rng.gen_range((self.page_min + self.page_max) / 2 + 1, self.page_max + 1);
            } else {
                chance += locality_increment;
            }
            if length_of_local != 0 {
                length_of_local -= 1;
                let request = rng.gen_range(lower_boundary, upper_boundary + 1);
                self.requests.push(request);
            } else {
                let request = rng.gen_range(self.page_min, self.page_max + 1);
                self.requests.push(request);
            }
            current_request += 1;
        }
    }

    pub fn set_first_frame(&mut self, first_frame: i32) {
        self.first_frame = first_frame;
    }

    pub fn set_last_frame(&mut self, last_frame: i32) {
        self.last_frame = last_frame;
    }

    pub fn get_last_frame(&self) -> i32 {
        self.last_frame
    }

    pub fn frames_no(&self) -> i32 {
        self.last_frame - self.first_frame + 1
    }

    pub fn pages_no(&self) -> i32 {
        self.page_max - self.page_min + 1
    }

    pub fn calculate_pff(&self) -> f32 {
        (self.page_faults as f32) / (self.time_window as f32)
    }
}
