/// An ELF (Executable and Linkable Format) parser library.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass { Elf32, Elf64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfEndian { Little, Big }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfType {
    None, Relocatable, Executable, Shared, Core,
}

#[derive(Debug, Clone)]
pub struct ElfHeader {
    pub class: ElfClass,
    pub endian: ElfEndian,
    pub elf_type: ElfType,
    pub machine: u16,
    pub entry_point: u64,
    pub ph_offset: u64,
    pub sh_offset: u64,
    pub ph_count: u16,
    pub sh_count: u16,
    pub sh_str_index: u16,
}

impl fmt::Display for ElfHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ELF {:?} {:?} type={:?} machine={} entry={:#x} ph={} sh={}",
            self.class, self.endian, self.elf_type, self.machine,
            self.entry_point, self.ph_count, self.sh_count
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    Null, Load, Dynamic, Interp, Note, Shlib, Phdr, Other(u32),
}

impl From<u32> for SegmentType {
    fn from(v: u32) -> Self {
        match v {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interp,
            4 => SegmentType::Note,
            5 => SegmentType::Shlib,
            6 => SegmentType::Phdr,
            _ => SegmentType::Other(v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub seg_type: SegmentType,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub flags: u32,
    pub align: u64,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name_index: u32,
    pub sh_type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub entsize: u64,
}

#[derive(Debug, Clone)]
pub struct ElfFile {
    pub header: ElfHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub section_names: HashMap<usize, String>,
}

impl ElfFile {
    pub fn parse_magic(data: &[u8]) -> Result<bool, String> {
        if data.len() < 4 {
            return Err("data too short".into());
        }
        Ok(&data[0..4] == &[0x7F, b'E', b'L', b'F'])
    }

    pub fn loadable_segments(&self) -> Vec<&ProgramHeader> {
        self.program_headers
            .iter()
            .filter(|ph| ph.seg_type == SegmentType::Load)
            .collect()
    }

    pub fn total_load_size(&self) -> u64 {
        self.loadable_segments().iter().map(|ph| ph.memsz).sum()
    }
}

impl fmt::Display for ElfFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for (i, ph) in self.program_headers.iter().enumerate() {
            writeln!(f, "PH[{}] {:?} offset={:#x} vaddr={:#x} filesz={:#x} memsz={:#x}",
                i, ph.seg_type, ph.offset, ph.vaddr, ph.filesz, ph.memsz)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_check() {
        assert_eq!(ElfFile::parse_magic(&[0x7F, b'E', b'L', b'F']), Ok(true));
        assert_eq!(ElfFile::parse_magic(&[0x00; 4]), Ok(false));
    }

    #[test]
    fn test_segment_type_from() {
        assert_eq!(SegmentType::from(1u32), SegmentType::Load);
        assert_eq!(SegmentType::from(99u32), SegmentType::Other(99));
    }
}
