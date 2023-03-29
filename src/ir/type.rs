#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    I64,
    U64,
    F64,
    I32,
    U32,
    F32,
    I16,
    U16,
    C16, //Char
    U8,
    I8,
    UPtr,
    IPtr,
    ObjRef,
    Bool,
}
impl Type {
    pub(crate) fn arthm_promote(&self) -> Type {
        match self {
            Self::I64 | Self::U64 | Self::F64 | Self::I32 | Self::U32 | Self::F32 => *self,
            Self::I16 | Self::I8 => Self::I32,
            Self::U16 | Self::U8 => Self::U32,
            _ => todo!("Type promotion for arithmetic operations on type {self:?} unhanded!"),
        }
    }
}
