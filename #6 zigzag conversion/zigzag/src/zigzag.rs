use std::cmp::max;

// each line has an established step pattern. this step pattern will be between one or two values.
// This can be programmed as the AddReg you see below and the max_add.  

struct AddReg {
    arr: [i32; 2],
    counter: usize,
}

impl AddReg {
    pub fn new(n: i32) -> AddReg {
        AddReg {
            arr: [n, 0],
            counter: 0,
        }
    }
    // get the value. switch the counter between 0 and 1. if value is zero then get the other
    // value.
    pub fn val(&mut self) -> i32 {
        let n = self.arr[self.counter];
        self.counter = (self.counter + 1) % 2;
        if n == 0 {self.arr[self.counter]} else {n}
    }
    // Move 2 between register and reset reg position.
    pub fn step(&mut self) {
        self.arr[0] -= 2;
        self.arr[1] += 2;
        self.counter = 0;
    }
}

pub fn convert(s: String, num_rows: i32) -> String {

    let s_len = s.len() as i32;
    let max_add = max(1, 2*(num_rows-1));
    let mut add_reg = AddReg::new(max_add);

    let mut c_s = String::new();
    
    for i in 0..num_rows {
        let mut index = i;
        for _ in 0..s_len {
            if index > s_len - 1 - i {break}
            println!("{}", index);
            c_s.push(s.chars().nth(index as usize).unwrap());
            index += add_reg.val();
        }
        add_reg.step();
    }
    c_s
}
