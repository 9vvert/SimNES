use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    LDA = 0xa9,
    Unknown,
}
