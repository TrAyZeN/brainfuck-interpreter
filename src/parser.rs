pub fn parse(program: &Vec<u8>) -> Vec<(char, u8)> {
    let mut tokens: Vec<(char, u8)> = Vec::new();
    let mut t = (program[0] as char, 0);
    let op: Vec<char> = vec![
        '>', '<', '+', '-',
        '.', ',', '[', ']'
    ];

    for o in program.iter().map(|x| *x as char) {
        match o {
            '['| ']' if o == t.0 => {
                tokens.push(t);
                t = (o, 1);
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
