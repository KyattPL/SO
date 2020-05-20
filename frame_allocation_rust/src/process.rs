use rand::prelude::*;

pub struct Process {
    page_min: i32,
    page_max: i32,
    pub requests: Vec<i32>,
    pub first_frame: i32,
    pub last_frame: i32,
}

impl Process {
    pub fn new(page_min: i32, page_max: i32) -> Process {
        Process {
            page_min,
            page_max,
            requests: vec![],
            first_frame: 0,
            last_frame: 0,
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

    pub fn lru_chunk(&mut self, frames: &mut [i32]) {
        let mut initializer = 1;
        let frames_len = frames.len();
        let requests_no = self.requests.len();
        let mut current_request = frames_len;
        while initializer <= frames_len {
            frames[(initializer - 1) as usize] =
                *self.requests.get((initializer - 1) as usize).unwrap();
            initializer += 1;
        }
        while current_request < requests_no {
            if !frames.contains(self.requests.get(current_request as usize).unwrap()) {
                let mut min = requests_no;
                let mut index = 0;
                let mut min_index = 0;
                let mut scan_index = current_request;
                while index < frames_len {
                    if frames[index as usize] == *self.requests.get(scan_index as usize).unwrap() {
                        if min > scan_index {
                            min = scan_index;
                            min_index = index;
                        }
                        index += 1;
                        scan_index = current_request;
                        continue;
                    }
                    scan_index -= 1;
                    if scan_index as i32 == -1 {
                        min_index = index;
                        break;
                    }
                }
                frames[min_index as usize] = *self.requests.get(current_request as usize).unwrap();
            }
            current_request += 1;
        }
    }
}
