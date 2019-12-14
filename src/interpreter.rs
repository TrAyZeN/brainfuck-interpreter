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

    pub fn run(&mut self, program: Vec<u8>) {
        let mut i = 0;

        let tokens = parser::parse(&program);
        // let loop_map = Interpreter::map_program(&tokens);

        while i < tokens.len() {
            match tokens[i].0 {
                '>' => self.increment_pointer(tokens[i].1 as usize),
                '<' => self.decrement_pointer(tokens[i].1 as usize),
                '+' => self.increment_memory(tokens[i].1),
                '-' => self.decrement_memory(tokens[i].1),
                '.' => self.print_char(tokens[i].1),
                ',' => self.read_char(tokens[i].1),
                '[' => self.open_loop(&mut i, &tokens),
                ']' => self.close_loop(&mut i),
                 _  => (),
            }

            i += 1;
        }
    }

    fn increment_pointer(&mut self, v: usize) {
        self.ptr += v;
    }

    fn decrement_pointer(&mut self, v: usize) {
        self.ptr -= v;
    }

    fn increment_memory(&mut self, v: u8) {
        self.memory[self.ptr] = self.memory[self.ptr].overflowing_add(v).0;
    }

    fn decrement_memory(&mut self, v: u8) {
        self.memory[self.ptr] = self.memory[self.ptr].overflowing_sub(v).0;
    }

    fn print_char(&mut self, v: u8) {
        for _ in 0..v {
            print!("{}", self.memory[self.ptr] as char);
        }
    }

    fn read_char(&mut self, v: u8) {    // TODO: implement read_char

    }

    fn open_loop(&mut self, i: &mut usize, tokens: &Vec<(char, u8)>) {
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

    fn jump_loop_end(&mut self, i: &mut usize, tokens: &Vec<(char, u8)>) {
        let mut c = 1;
        while c > 0 {
            *i += 1;
            match tokens[*i].0 {
                '[' => c += 1,
                ']' => c -= 1,
                _ => (),
            }
        }
    }

    fn map_program(tokens: &Vec<(char, u8)>) -> HashMap<usize, usize> {
        let mut loop_stack = Vec::new();
        let mut loop_map = HashMap::new();

        for (k, (o, _)) in tokens.iter().enumerate() {
            match o {
                '[' => loop_stack.push(k),
                ']' if loop_stack.last().is_some() => {
                    loop_map.insert(loop_stack.pop().unwrap(), k);
                },
                _ => ()
            }
        }

        loop_map
    }
}
