pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn next_item(&mut self, program: &Vec<u8>) -> u8 {
        let item = program[self.program_counter as usize];
        self.program_counter += 1;
        item
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.status = if result == 0 {
            self.status | 0b0000_0010
        } else {
            self.status & 0b1111_1101
        };
        self.status = if result & 0b1000_0000 == 0 {
            self.status & 0b0111_1111
        } else {
            self.status | 0b1000_0000
        };
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(0b01);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
        loop {
            let opscode = self.next_item(&program);

            match opscode {
                0x00 => {
                    return;
                }
                0xA9 => {
                    let param = self.next_item(&program);
                    self.lda(param);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                _ => todo!(),
            }
        }
    }
}
