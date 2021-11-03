use crate::registers::{ Registers, Flag };

trait Carry {
    fn carry_add(&self, rhs: u8) -> (u8, bool, bool);
    fn carry_sub(&self, rhs: u8) -> (u8, bool, bool);
}

impl Carry for u8 {
    fn carry_add(&self, rhs: u8) -> (u8, bool, bool) {
        let r = self.wrapping_add(rhs);
        // https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
        let h = ((*self & 0x0F) + (rhs & 0x0F)) & 0x10 == 0x10;
        let c = (*self as u16) + (rhs as u16) > 0xFF;
        (r, h, c)
    }

    fn carry_sub(&self, rhs: u8) -> (u8, bool, bool) {
        let r = self.wrapping_sub(rhs);
        let h = (*self & 0x0F) < (rhs & 0x0F);
        let c = *self < rhs;
        (r, h, c)
    }
}

// 8-bit Functions

// Stores the result of ADD n in A and sets flags
pub fn alu_add(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_add(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
    reg.a = r.0;
}

// Stores the result of SUB n in A and sets flags
pub fn alu_sub(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_sub(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
    reg.a = r.0;
}

// Returns the result of INC n and sets flags
pub fn alu_inc(reg: &mut Registers, n: u8) -> u8 {
    let r = n.carry_add(1);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, r.1);
    r.0
}

// Returns the result of DEC n and sets flags
pub fn alu_dec(reg: &mut Registers, n: u8) -> u8 {
    let r = n.carry_sub(1);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    r.0
}

// Stores the reult of OR n in A and sets flags
pub fn alu_or(reg: &mut Registers, n: u8) {
    reg.a = reg.a | n;
    reg.set_flag(Flag::Z, reg.a == 0);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, false);
    reg.set_flag(Flag::C, false);
}

// Sets flags from CP n
pub fn alu_cp(reg: &mut Registers, n: u8) {
    let r = reg.a.carry_sub(n);
    reg.set_flag(Flag::Z, r.0 == 0);
    reg.set_flag(Flag::N, true);
    reg.set_flag(Flag::H, r.1);
    reg.set_flag(Flag::C, r.2);
}

// 16-bit Functions

// Adds n to HL and sets flags
pub fn alu_add_hl(reg: &mut Registers, n: u16) {
    let r = reg.hl().wrapping_add(n);
    reg.set_flag(Flag::N, false);
    reg.set_flag(Flag::H, (reg.hl() & 0x07FF) + (n & 0x07FF) > 0x07FF);
    reg.set_flag(Flag::C, ((reg.hl() as u32) + (n as u32)) > 0xFFFF);
    reg.set_hl(r);
}