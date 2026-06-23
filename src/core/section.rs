use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::SabError;

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SectionKind {
    Null = 0x0,
    Data = 0x1,
    Code = 0x2,
    RoData = 0x3,
    UninitializedData = 0x4,
    Other = 0x5,
    Tag = 0x6,
}

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct SectionFlags: u8 {
        const R = 0b0000_0001;
        const W = 0b0000_0010;
        const X = 0b0000_0100;
        const NO_BITS = 0b0000_1000;
        const ALLOC = 0b0001_0000;

        const RWA = Self::R.bits() | Self::W.bits() | Self::ALLOC.bits();
        
        const RXA = Self::R.bits() | Self::X.bits() | Self::ALLOC.bits();
        
        const RA = Self::R.bits() | Self::ALLOC.bits();

        const NARW = Self::NO_BITS.bits() | Self::ALLOC.bits() | Self::R.bits() | Self::W.bits();
    } 
}

#[derive(Debug)]
pub struct SectionEntry {
    pub vaddr: u64,
    pub data_offset: u64,
    pub data_len: u64,
    pub name: u32,
    pub name_len: u32,
    pub kind: SectionKind,
    pub flags: SectionFlags,
    pub _reserved: [u8; 6]
}

impl Default for SectionEntry {
    fn default() -> Self {
        Self {
            vaddr: 0,
            data_offset: 0,
            data_len: 0,
            name: 0,
            name_len: 0,
            kind: SectionKind::Null,
            flags: SectionFlags::empty(),
            _reserved: [0_u8; 6]
        }
    }
}

impl SectionEntry {
    pub fn read<T: ReadBytesExt>(stream: &mut T) -> Result<Self, SabError>  {
        let vaddr = stream.read_u64::<LittleEndian>()?;

        let data_offset = stream.read_u64::<LittleEndian>()?;
        let data_len = stream.read_u64::<LittleEndian>()?;

        let name = stream.read_u32::<LittleEndian>()?;
        let name_len = stream.read_u32::<LittleEndian>()?;

        let kind = match stream.read_u8()? {
            0 => SectionKind::Null,
            1 => SectionKind::Data,
            2 => SectionKind::Code,
            3 => SectionKind::RoData,
            4 => SectionKind::UninitializedData,
            5 => SectionKind::Other,
            6 => SectionKind::Tag,
            n => return Err(SabError::DecodeError(format!("Invalid section kind {n}"))),
        };

        let flags = SectionFlags::from_bits_truncate(stream.read_u8()?);
    
        if flags.contains(SectionFlags::W | SectionFlags::X) {
            return Err(SabError::DecodeError("Ilegal section flags".to_string()));
        }

        
        stream.read(&mut [0_u8; 6])?;


        Ok(Self {
            vaddr,
            data_offset,
            data_len,
            name,
            name_len,
            kind,
            flags,
            _reserved: [0_u8; 6],
        })
    }
    pub fn write<T: WriteBytesExt>(self, stream: &mut T) -> Result<(), SabError> {
        if self.flags.contains(SectionFlags::W | SectionFlags::X) {
            return Err(SabError::DecodeError("Ilegal section flags".to_string()));
        }

        stream.write_u64::<LittleEndian>(self.vaddr)?;
        stream.write_u64::<LittleEndian>(self.data_offset)?;
        stream.write_u64::<LittleEndian>(self.data_len)?;

        stream.write_u32::<LittleEndian>(self.name)?;
        stream.write_u32::<LittleEndian>(self.name_len)?;

        stream.write_u8(self.kind as u8)?;
        stream.write_u8(self.flags.bits())?;
        stream.write(&self._reserved)?;

        Ok(())

    }
}
