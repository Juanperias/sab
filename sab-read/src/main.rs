use std::io::Cursor;

use sab::{core::section::{SectionFlags, SectionKind}, obj::{reader::SabObjReader, writer::SabObjWriter}};

fn main() {
    let mut v: Vec<u8> = Vec::new();
    let mut c = Cursor::new(&mut v);

    let mut obj = SabObjWriter::new();

    obj.create_section(0, "text".as_bytes(), vec![], SectionKind::Code, SectionFlags::RXA);
    obj.create_section(0, "metadata".as_bytes(), b"Hola esta es informacion adicional".to_vec(), SectionKind::Other, SectionFlags::R);
    obj.create_section(0, "data".as_bytes(), "Hola mundo!".as_bytes().to_vec(), SectionKind::Data, SectionFlags::RWA);

    let file = obj.to_file(sab::core::header::Endian::Little, sab::core::header::Machine::Riscv32);
    
    
    println!("{:?}", file);
    file.write(&mut c).unwrap();
    c.set_position(0);

    let read = SabObjReader::new(&mut c).unwrap();

    println!("{:?}", read.sections());

    println!("Hello, world!");
}
