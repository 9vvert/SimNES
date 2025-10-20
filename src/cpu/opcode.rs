use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    BRK = 0x00,
    LDA = 0xa9,
    TAX = 0xaa,
    INX = 0xe8,

    Unknown,
}
// u8 to Opcode
impl Opcode {
    pub fn from_u8(val: u8) -> Opcode {
        Opcode::try_from_primitive(val).unwrap_or(Opcode::Unknown)
    }
}
