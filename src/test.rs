use super::*;

mod tests {
    use super::*;

    #[test]
    fn test_write_byte() {
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        let a = 0xFABA;
        let b = 0xFF;
        write_byte(a, b, &mut mem);
        assert_eq!(b, read_byte(a, &mut mem));
    }

    #[test]
    fn test_alu_cp() {
        let mut reg = Registers::new();
        reg.a =		0b11010010;
        let b: u8 =	0b00101001;
        alu_cp(&mut reg, b);
		assert!(!reg.get_flag(Flag::Z));
		assert!(reg.get_flag(Flag::N));
		assert!(reg.get_flag(Flag::H));
		assert!(!reg.get_flag(Flag::C));
    }

	#[test]
	fn test_alu_cp_2() {
		let mut reg = Registers::new();
		reg.a =		0b01011001;
        let b: u8 =	0b10000100;
        alu_cp(&mut reg, b);
		assert!(!reg.get_flag(Flag::Z));
		assert!(reg.get_flag(Flag::N));
		assert!(!reg.get_flag(Flag::H));
		assert!(reg.get_flag(Flag::C));
	}

	#[test]
	fn test_alu_add_hl() {
		let mut reg = Registers::new();
        reg.set_hl(0x0FFF);
        let n =	0x0FFE;
		alu_add_hl(&mut reg, n);
		assert!(!reg.get_flag(Flag::N));
		assert!(reg.get_flag(Flag::H));
		assert!(!reg.get_flag(Flag::C));
	}

	#[test]
	fn test_alu_add_hl_2() {
		let mut reg = Registers::new();
        reg.set_hl(0xF000);
        let n =	0xFFFF;
		alu_add_hl(&mut reg, n);
		assert!(!reg.get_flag(Flag::N));
		assert!(!reg.get_flag(Flag::H));
		assert!(reg.get_flag(Flag::C));
	}
}