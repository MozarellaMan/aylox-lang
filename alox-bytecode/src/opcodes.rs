use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum Op {
    Return = 0,
    Constant,
}

impl Op {
    pub fn u8(self) -> u8 {
        self as u8
    }
}
