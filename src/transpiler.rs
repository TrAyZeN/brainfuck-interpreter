use std::io;
use std::fs::File;
use std::io::{BufWriter, Write};
use super::parser;

pub fn to_c(program: &[u8], filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut file_writer = BufWriter::new(file);
    file_writer.write_all(b"#include \"stdio.h\"\n")?;
    file_writer.write_all(b"int main(){unsigned char a[3000]={};unsigned char *p=a;")?;

    for (o, v) in parser::parse(program) {
        match o {
            '>' => file_writer.write_all(format!("p+={};", v).as_bytes()),
            '<' => file_writer.write_all(format!("p-={};", v).as_bytes()),
            '+' => file_writer.write_all(format!("*p+={};", v).as_bytes()),
            '-' => file_writer.write_all(format!("*p-={};", v).as_bytes()),
            '.' => file_writer.write_all("putchar(*p);".repeat(v as usize).as_bytes()),
            ',' => file_writer.write_all("*p=getchar();".repeat(v as usize).as_bytes()),
            '[' => file_writer.write_all("while(*p){".repeat(v as usize).as_bytes()),
            ']' => file_writer.write_all("}".repeat(v as usize).as_bytes()),
             _  => Ok(()),
        }?
    }
    file_writer.write_all(b"}")?;

    Ok(())
}

pub fn to_rust(program: &[u8], filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut file_writer = BufWriter::new(file);
    file_writer.write_all(b"fn a(x:u8,y:u8)->u8{x.overflowing_add(y).0}")?;
    file_writer.write_all(b"fn s(x:u8,y:u8)->u8{x.overflowing_sub(y).0}")?;
    file_writer.write_all(b"fn main(){let mut t=[0u8;3000];let mut p=0usize;")?;

    for (o, v) in parser::parse(program) {
        match o {
            '>' => file_writer.write_all(format!("p+={};", v).as_bytes()),
            '<' => file_writer.write_all(format!("p-={};", v).as_bytes()),
            '+' => file_writer.write_all(format!("t[p]=a(t[p],{});", v).as_bytes()),
            '-' => file_writer.write_all(format!("t[p]=s(t[p],{});", v).as_bytes()),
            '.' => file_writer.write_all("print!(\"{}\",t[p]as char);".repeat(v as usize).as_bytes()),
            ',' => file_writer.write_all("();".repeat(v as usize).as_bytes()),   // TODO: implement getchar
            '[' => file_writer.write_all("while t[p]!=0{".repeat(v as usize).as_bytes()),
            ']' => file_writer.write_all("}".repeat(v as usize).as_bytes()),
             _  => Ok(()),
        }?
    }
    file_writer.write_all(b"}")?;

    Ok(())
}
