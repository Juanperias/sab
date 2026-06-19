use std::{fmt::Write, io::Read};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{SabError, core::{header::SabHeader, section::SectionEntry}};

pub struct SabFile {
    pub header: SabHeader,
    pub section_table: Vec<SectionEntry>,
    pub data: Vec<u8>,
}

impl SabFile {
    pub fn read<T: ReadBytesExt>(stream: &mut T) -> Result<Self, SabError> {
        let header = SabHeader::read(stream)?;
        let section_table: Vec<SectionEntry> = (0..header.section_num).into_iter().map(|_| SectionEntry::read(stream)).collect::<Result<Vec<SectionEntry>, _>>()?;
        
        let mut data = Vec::with_capacity(header.data_size as usize);
        stream.take(header.data_size).read_to_end(&mut data)?;

        Ok(Self {
            header,
            section_table,
            data,
        })
    }
    pub fn write<T: WriteBytesExt>(self, stream: &mut T) -> Result<(), SabError> {
        self.header.write(stream)?;
        for entry in self.section_table { 
            entry.write(stream)?;
        }
        stream.write(&self.data)?;

        Ok(())
    }
}
