use std::collections::{HashMap, hash_map::Values};

use byteorder::ReadBytesExt;

use crate::{SabError, core::{file::SabFile, header::SabHeader, section::{SectionEntry, SectionFlags, SectionKind}}};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub vaddr: u64,
    pub data: Vec<u8>,
    pub flags: SectionFlags,
    pub kind: SectionKind,
}

pub struct SabObjReader(HashMap<String, Section>);

impl SabObjReader {
    pub fn new<T: ReadBytesExt>(stream: &mut T) -> Result<Self, SabError> {
        let file = SabFile::read(stream)?;

        let mut map = HashMap::new();

        for section in file.section_table.iter().skip(1) {
            let name_index = section.name as usize;
            let data_offset = (section.data_offset as usize) - (std::mem::size_of::<SabHeader>() + std::mem::size_of::<SectionEntry>() * file.header.section_num as usize);

            let name = String::from_utf8(file.data[name_index..(section.name_len as usize) + name_index].to_vec()).unwrap();
            let data = file.data[data_offset..(section.data_len as usize) + data_offset].to_vec();


            let _ = map.insert(name.clone(), Section {
                name,
                vaddr: section.vaddr,
                data,
                flags: section.flags.clone(),
                kind: section.kind.clone(),
            });
        }

        Ok(Self(map))
    }
    pub fn sections(&self) -> Values<'_, String, Section>  {
        self.0.values()
    }
}


