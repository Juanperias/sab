
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::SabError;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Endian {
    Little = 0,
    Big = 1,
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Machine {
    None = 0,
    Riscv32 = 1,
    Riscv64 = 2,
    Arm32 = 3,
    Arm64 = 4,
    Jmc32 = 5,
}

#[derive(Debug)]
#[repr(C, align(8))]
pub struct SabHeader {
    pub magic: [u8; 3],
    pub version: u8,
    pub endian: Endian,
    pub machine: Machine,
    pub entry_symbol: u16,
    pub section_num: u32,
    pub padding: [u8; 4],
    pub symbol_num: u64,
    pub relocation_num: u64, 
    pub data_size: u64,
}

impl Default for SabHeader {
    fn default() -> Self {
        Self {
            magic: *b"SAB",
            version: 1,
            endian: Endian::Little,
            machine: Machine::None,
            entry_symbol: 0,
            section_num: 0, 
            padding: [0, 0, 0, 0],
            symbol_num: 0,
            relocation_num: 0,
            data_size: 0,
        }
    }
}

impl SabHeader {
    pub fn read<T: ReadBytesExt>(stream: &mut T) -> Result<Self, SabError>  {
        let mut magic = [0_u8; 3];

        stream.read(&mut magic)?;

        if &magic != b"SAB" {
            return Err(SabError::DecodeError(format!("Invalid magic, {:?}", magic))) 
        }

        let version = stream.read_u8()?;

        if version != 1 {
            return Err(SabError::DecodeError(format!("Invalid version {version}")));
        }

        let endian = match stream.read_u8()? {
            0 => Endian::Little,
            1 => Endian::Big,
            e => return Err(SabError::DecodeError(format!("Invalid endian, {e}"))),
        };

        let machine = match stream.read_u8()? {
           0 => return Err(SabError::DecodeError(format!("Executable does not have any machine"))),
           1 => Machine::Riscv32,
           2 => Machine::Riscv64,
           3 => Machine::Arm32,
           4 => Machine::Arm64,
           5 => Machine::Jmc32,
           n => return Err(SabError::DecodeError(format!("Invalid machine, {n}")))
        };

        let entry_symbol = stream.read_u16::<LittleEndian>()?;

        let section_num = stream.read_u32::<LittleEndian>()?;

        let mut padding = [0_u8; 4];
        stream.read(&mut padding)?;

        let symbol_num = stream.read_u64::<LittleEndian>()?;
        let relocation_num = stream.read_u64::<LittleEndian>()?;

        let data_size = stream.read_u64::<LittleEndian>()?;

        Ok(Self {
            magic,
            version,
            endian,
            machine,
            entry_symbol,
            section_num,
            padding,
            symbol_num,
            relocation_num,
            data_size
        })
    }
    pub fn write<T: WriteBytesExt>(self, stream: &mut T) -> Result<(), SabError> {
        stream.write(&self.magic)?; 
        stream.write_u8(self.version)?;
        stream.write_u8(self.endian as u8)?;
        stream.write_u8(self.machine as u8)?;
        stream.write_u16::<LittleEndian>(self.entry_symbol)?;
        stream.write_u32::<LittleEndian>(self.section_num)?;
        stream.write(&self.padding)?;
        stream.write_u64::<LittleEndian>(self.symbol_num)?;
        stream.write_u64::<LittleEndian>(self.relocation_num)?;
        stream.write_u64::<LittleEndian>(self.data_size)?;

        Ok(())
    }
}
