
use crate::core::{file::SabFile, header::{Endian, Machine, SabHeader}, section::{SectionEntry, SectionFlags, SectionKind}};

struct Section {
    raw: SectionEntry,
    data: Vec<u8>,
}

pub struct SabObjWriter {
    pub sections: Vec<Section>,
    pub name_section: Section, 
}


impl SabObjWriter {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            name_section: Section {
                raw: SectionEntry {
                    vaddr: 0,
                    data_offset: 0, 
                    data_len: 0,
                    name: 0,
                    name_len: 0,
                    kind: crate::core::section::SectionKind::Tag,
                    flags: SectionFlags::R,
                    ..Default::default()
                },
                data: Vec::new(),
            },
        }
    }
    pub fn create_section(&mut self, vaddr: u64, name: &[u8], content: Vec<u8>, kind: SectionKind, flags: SectionFlags) {
        self.name_section.data.extend_from_slice(name);

        self.sections.push(Section { raw: SectionEntry {
            vaddr,
            kind,
            flags,
            data_offset: 0,
            data_len: content.len() as u64,
            name_len: name.len() as u32,
            name: 0,
            ..Default::default()
        }, data: content });
    }

    pub fn to_file(self, endian: Endian, machine: Machine) -> SabFile {
        let data_len = self.sections.iter().fold(0, |acc, f| {
            acc + f.data.len()
        }) + self.name_section.data.len();





        let header = SabHeader {
            endian,
            machine,
            entry_symbol: 0,
            section_num: (self.sections.len() + 1) as u32,
            symbol_num: 0,
            relocation_num: 0,
            data_size: data_len as u64,
            ..Default::default()
        };

        let mut data_index = std::mem::size_of::<SabHeader>() + std::mem::size_of::<SectionEntry>() * (self.sections.len() + 1) + self.name_section.data.len();

        let mut name_index = 0;

        let mut data = Vec::new();


        let mut name = self.name_section;

        name.raw.data_offset = (std::mem::size_of::<SabHeader>() + std::mem::size_of::<SectionEntry>() * (self.sections.len() + 1)) as u64;
        name.raw.data_len = name.data.len() as u64;

        
        data.extend(name.data);

        let mut section_table = Vec::new();


        section_table.push(name.raw);

        for mut section in self.sections {
            section.raw.data_offset = data_index as u64;
            section.raw.name = name_index as u32; 
        

            data_index += section.data.len();
            name_index += section.raw.name_len as usize;
        
            section_table.push(section.raw);
            data.extend(section.data);
        }

       




        SabFile { header, section_table, data }
    }
}
