use std::collections::HashMap;
use super::parser;

pub struct Interpreter {
    ptr: usize,
    memory: Vec<u8>,
    loop_stack: Vec<usize>
}

impl Interpreter {
    pub fn new(memory_size: usize) -> Interpreter {
        Interpreter {
            ptr: 0,
            memory: vec![0; memory_size],
            loop_stack: Vec::new()
        }
    }

    pub fn run(&mut self, program: &[u8]) {
        let mut i = 0;

        let tokens = parser::parse(&program);
        // let loop_map = Interpreter::map_program(&tokens);

        while i < tokens.len() {
            match tokens[i].0 {
                b'>' => self.increment_pointer(tokens[i].1 as usize),
                b'<' => self.decrement_pointer(tokens[i].1 as usize),
                b'+' => self.increment_memory(tokens[i].1),
                b'-' => self.decrement_memory(tokens[i].1),
                b'.' => self.print_char(tokens[i].1),
                b',' => self.read_char(tokens[i].1),
                b'[' => self.open_loop(&mut i, &tokens),
                b']' => self.close_loop(&mut i),
                 _  => (),
            }

            i += 1;
        }
    }

    fn increment_pointer(&mut self, count: usize) {
        self.ptr += count;
    }

    fn decrement_pointer(&mut self, count: usize) {
        self.ptr -= count;
    }

    fn increment_memory(&mut self, count: u8) {
        self.memory[self.ptr] = self.memory[self.ptr].overflowing_add(count).0;
    }

    fn decrement_memory(&mut self, count: u8) {
        self.memory[self.ptr] = self.memory[self.ptr].overflowing_sub(count).0;
    }

    fn print_char(&mut self, count: u8) {
        for _ in 0..count {
            print!("{}", self.memory[self.ptr] as char);
        }
    }

    fn read_char(&mut self, count: u8) {    // TODO: implement read_char

    }

    fn open_loop(&mut self, i: &mut usize, tokens: &Vec<(u8, u8)>) {
        if self.memory[self.ptr] == 0 {
            self.jump_loop_end(i, tokens);
        }
        else if self.loop_stack.last().is_none()
        || *i != *self.loop_stack.last().unwrap() {
            self.loop_stack.push(*i);
        }
    }

    fn close_loop(&mut self, i: &mut usize) {
        if self.memory[self.ptr] == 0 {
            self.loop_stack.pop();
        }
        else if self.loop_stack.last().is_some() {
            self.jump_loop_begin(i);
        }
    }

    fn jump_loop_begin(&mut self, i: &mut usize) {
        *i = *self.loop_stack.last().unwrap();
    }

    fn jump_loop_end(&mut self, i: &mut usize, tokens: &Vec<(u8, u8)>) {
        let mut counter = 1;
        while counter > 0 {
            *i += 1;
            match tokens[*i].0 {
                b'[' => counter += 1,
                b']' => counter -= 1,
                _ => (),
            }
        }
    }

    fn map_program(tokens: &Vec<(u8, u8)>) -> HashMap<usize, usize> {
        let mut loop_stack = Vec::new();
        let mut loop_map = HashMap::new();

        for (k, (operation, _)) in tokens.iter().enumerate() {
            match operation {
                b'[' => loop_stack.push(k),
                b']' if loop_stack.last().is_some() => {
                    loop_map.insert(loop_stack.pop().unwrap(), k);
                },
                _ => ()
            }
        }

        loop_map
    }
}
