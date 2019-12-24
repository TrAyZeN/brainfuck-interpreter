pub fn parse(program: &[u8]) -> Vec<(u8, u8)> {
    let mut tokens: Vec<(u8, u8)> = Vec::new();
    let mut t = (program[0], 0);
    let op = [
        b'>', b'<', b'+', b'-',
        b'.', b',', b'[', b']'
    ];

    for o in program.iter() {
        match *o {
            b'['| b']' if *o == t.0 => {
                tokens.push(t);
                t = (*o, 1);
            },
            o if o == t.0 => {
                t.1 += 1;
            },
            o if op.contains(&o) => {
                tokens.push(t);
                t = (o, 1);
            },
            _ => ()
        }
    }
    tokens.push(t);

    tokens
}
