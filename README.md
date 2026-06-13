# ELF Parser

**A parser for the ELF (Executable and Linkable Format) binary format** — the standard executable format on Linux, used by every compiled Rust, C, and C++ program. This library parses ELF headers, program headers (segments), and section headers, enabling binary introspection for debuggers, profilers, and linkers.

## Why It Matters

ELF is the binary format of the Linux world. When you compile `main.rs`, the output is an ELF file. When you run `ldd ./program`, it reads ELF dependencies. When GDB sets a breakpoint, it parses ELF debug sections. When a bootloader loads a kernel, it processes ELF program headers.

Understanding ELF structure is essential for:
- **Reverse engineering** — Analyzing malware, understanding binary structure
- **Building debuggers** — Reading `.debug_info` sections (see debug-dwarf)
- **Profiling** — Mapping instruction pointers to source locations
- **Linkers** — Combining object files, resolving symbols
- **Bootloaders** — Loading kernel images into memory
- **Dynamic linking** — Finding shared library dependencies

**ELF structure:** An ELF file has three levels:
1. **ELF Header** — Magic bytes (0x7F ELF), architecture (32/64-bit), endianness, type (executable, shared library, core dump), entry point address, and offsets to the program/section header tables
2. **Program Headers** — Describe memory segments (LOAD, DYNAMIC, INTERP, etc.). The OS loader uses these to map the file into virtual memory.
3. **Section Headers** — Describe named sections (.text, .data, .bss, .rodata, .debug_info). Used by linkers and debuggers.

## How It Works

The library models the ELF format as Rust types:

**`ElfHeader`** — The top-level header containing:
- `class: ElfClass` — `Elf32` or `Elf64` (determines pointer sizes)
- `endian: ElfEndian` — Little or Big endian
- `elf_type: ElfType` — Relocatable (.o), Executable, Shared (.so), or Core dump
- `entry_point: u64` — Where execution begins (the `_start` symbol)
- `ph_offset / sh_offset` — Byte offsets to program/section header tables

**`ProgramHeader`** — Memory segment descriptor:
- `SegmentType` — Null, Load (mapped into memory), Dynamic (dynamic linking info), Interp (dynamic linker path), Note (metadata)
- `vaddr` — Virtual address where segment is loaded
- `filesz / memsz` — Size in file vs. memory (.bss sections are zero-filled beyond filesz)

**`SectionHeader`** — Named section descriptor:
- `.text` — Executable code
- `.data` — Initialized global variables
- `.bss` — Uninitialized globals (zero-filled)
- `.rodata` — Read-only data (string literals, constants)
- `.debug_*` — DWARF debugging information

**Magic validation:** `parse_magic` checks for the ELF magic number `0x7F 'E' 'L' 'F'` at byte offset 0. This is how the OS kernel identifies ELF files (vs. scripts with `#!` or other binary formats).

**Loadable segments:** The `loadable_segments()` method filters for `PT_LOAD` segments — these are the ones the OS maps into the process's virtual address space. `total_load_size()` gives the total memory footprint.

## Quick Start

```rust
use elf_parser::{ElfFile, ElfClass, ElfType};

// Read an ELF binary
let data = std::fs::read("/bin/ls").unwrap();

// Check magic bytes
let is_elf = ElfFile::parse_magic(&data).unwrap();
assert!(is_elf);

// (Full parsing would populate header, program_headers, section_headers)
// let elf = ElfFile::parse(&data).unwrap();
// println!("{}", elf.header);
// for seg in elf.loadable_segments() {
//     println!("LOAD: vaddr={:#x} memsz={:#x}", seg.vaddr, seg.memsz);
// }
```

## API

### `ElfFile`
- `parse_magic(data: &[u8]) -> Result<bool, String>` — Check ELF magic bytes
- `loadable_segments() -> Vec<&ProgramHeader>` — Filter PT_LOAD segments
- `total_load_size() -> u64` — Sum of all loadable segment memory sizes

### Types
- `ElfHeader` — class, endian, type, machine, entry_point, offsets
- `ProgramHeader` — seg_type, offset, vaddr, paddr, filesz, memsz, flags, align
- `SectionHeader` — name_index, sh_type, flags, addr, offset, size
- `ElfClass` — `Elf32` / `Elf64`
- `ElfEndian` — `Little` / `Big`
- `ElfType` — `None`, `Relocatable`, `Executable`, `Shared`, `Core`
- `SegmentType` — `Null`, `Load`, `Dynamic`, `Interp`, `Note`, `Shlib`, `Phdr`, `Other(u32)`

## Architecture Notes

This library provides binary introspection for SuperInstance's systems tooling — powering debuggers, profilers, and binary analysis tools. It complements the debug-dwarf library (which parses the `.debug_line` section that this parser locates).

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
