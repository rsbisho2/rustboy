pub type Memory = [u8; 0xFFFF + 1];

// Read byte from memory
pub fn read_byte(adr: u16, mem: &[u8]) -> u8 {
    print_debug("Read byte", adr);

    mem[adr as usize]
}

// Write word to memory
pub fn write_byte(adr: u16, val: u8, mem: &mut [u8]) {
    print_debug("Write byte", adr);

    mem[adr as usize] = val
}

// Read word from memory (lil' endian?)
pub fn read_word(adr: u16, mem: &[u8]) -> u16 {
    print_debug("Read word", adr);
    
    mem[adr as usize] as u16 | ((mem[(adr + 1) as usize] as u16) << 8)
}

// Write word to memory
pub fn write_word(adr: u16, val: u16, mem: &mut [u8]) {
    print_debug("Write word", adr);

    mem[adr as usize] = (val & 0x00FF) as u8;
    mem[(adr + 1) as usize] = (val >> 8) as u8;
}

fn print_debug(label: &str, adr: u16) {
    println!("{} ({:#04x}) - {}",
        label,
        adr,
        match adr {
            0x0000 ..= 0x7FFF => "ROM",
            0x8000 ..= 0x9FFF => "VRAM",
            0xA000 ..= 0xBFFF => "RAM",
            0xC000 ..= 0xDFFF => "WRAM",
            0xE000 ..= 0xFDFF => panic!("byte written to forbidden memory, {:#04}", adr),
            0xFE00 ..= 0xFE9F => "OAM",
            0xFEA0 ..= 0xFEFF => panic!("byte written to forbidden memory, {:#04}", adr),
            0xFF00 ..= 0xFF7F => "IO",
            0xFF80 ..= 0xFFFE => "HRAM",
            0xFFFF => "IE"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_byte() {
        let mut mem: Memory = [0; 0xFFFF + 1];
        let adr = 0xFFFE;
        let val = 0xFF;
        write_byte(adr, val, &mut mem);
        assert_eq!(val, read_byte(adr, &mut mem));
    }
}