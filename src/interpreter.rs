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
        let loop_map = Interpreter::map_program(&program);
        // let loop_stack = Vec::new();

        while i < tokens.len() {
            match tokens[i].0 {
                '>' => self.increment_pointer(tokens[i].1 as usize),
                '<' => self.decrement_pointer(tokens[i].1 as usize),
                '+' => self.increment_memory(tokens[i].1),
                '-' => self.decrement_memory(tokens[i].1),
                '.' => self.print_char(tokens[i].1),
                ',' => self.read_char(tokens[i].1),
                '[' => self.open_loop(&mut i, &loop_map),
                ']' => self.close_loop(&mut i),
                 _  => (),
            }

            i += 1;
        }
        // while i < program.len() {
        //     match program[i] as char {
        //         '>' => self.increment_pointer(1),
        //         '<' => self.decrement_pointer(1),
        //         '+' => self.increment_memory(1),
        //         '-' => self.decrement_memory(1),
        //         '.' => self.print_char(),
        //         ',' => self.read_char(),
        //         '[' => self.open_loop(&mut i, &loop_map),
        //         ']' => self.close_loop(&mut i),
        //          _  => (),
        //     }

        //     i += 1;
        // }
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

    fn read_char(&mut self, v: u8) {

    }

    fn open_loop(&mut self, i: &mut usize, loop_map: &HashMap<usize, usize>) {
        if self.memory[self.ptr] == 0 {
            *i = match loop_map.get(&self.ptr) {
                Some(v) => *v,
                None    => *i,
            };
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
            *i = *self.loop_stack.last().unwrap();
        }
    }

    fn map_program(program: &Vec<u8>) -> HashMap<usize, usize> {
        let mut loop_stack = Vec::new();
        let mut loop_map = HashMap::new();

        for (p, c) in program.iter().enumerate() {
            if *c == '[' as u8 {
                loop_stack.push(p);
            }
            else if *c == ']' as u8 && loop_stack.last().is_some() {
                loop_map.insert(loop_stack.pop().unwrap(), p);
            }
        }

        loop_map
    }
}
