pub fn parse(program: &[u8]) -> Vec<(u8, u8)> {
    let mut tokens: Vec<(u8, u8)> = Vec::new();
    let mut t = (program[0], 0);

    for operation in program.iter() {
        match *operation {
            b'['| b']' if *operation == t.0 => {
                tokens.push(t);
                t = (*operation, 1);
            },
            operation if operation == t.0 => {
                t.1 += 1;
            },
              b'>' | b'<' | b'+' | b'-'
            | b'.' | b',' | b'[' | b']' => {
                tokens.push(t);
                t = (*operation, 1);
            },
            _ => ()
        }
    }
    tokens.push(t);

    tokens
}
