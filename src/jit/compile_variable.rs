use crate::Type;
use inkwell::values::{BasicValue, BasicValueEnum, FloatValue, IntValue, PointerValue};
#[derive(Clone, Copy)]
pub(crate) enum Variable<'a> {
    Int(IntValue<'a>),
    UInt(IntValue<'a>),
    Float(FloatValue<'a>),
    Pointer(PointerValue<'a>),
}
impl<'a> Variable<'a> {
    pub(crate) fn as_int(&self) -> Option<IntValue<'a>> {
        if let Self::Int(int) = self {
            Some(*int)
        } else {
            None
        }
    }
    pub(crate) fn as_any_int(&self) -> Option<IntValue<'a>> {
        if let Self::Int(int) | Self::UInt(int) = self {
            Some(*int)
        } else {
            None
        }
    }
    pub fn as_uint(&self) -> Option<IntValue<'a>> {
        if let Self::UInt(int) = self {
            Some(*int)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<FloatValue<'a>> {
        if let Self::Float(float) = self {
            Some(*float)
        } else {
            None
        }
    }
    pub fn matching_int(&self, val: IntValue<'a>) -> Self {
        match self {
            Self::Int(_) => Self::Int(val),
            Self::UInt(_) => Self::UInt(val),
            _ => panic!("Variable {val} is not an integer!"),
        }
    }
    pub fn from_bve(bve: BasicValueEnum<'a>) -> Self {
        match bve {
            BasicValueEnum::IntValue(int) => Self::Int(int),
            BasicValueEnum::FloatValue(f) => Self::Float(f),
            BasicValueEnum::PointerValue(p) => Self::Pointer(p),
            _ => todo!("Can't convert {bve:?} to a Variable IR!"),
        }
    }
    pub fn from_bve_typed(bve: BasicValueEnum<'a>, t: &Type) -> Self {
        match t {
            Type::I64 | Type::I32 | Type::I16 | Type::I8 => Self::Int(bve.into_int_value()),
            Type::U64 | Type::U32 | Type::U16 | Type::U8 => Self::UInt(bve.into_int_value()),
            Type::F64 | Type::F32 => Self::Float(bve.into_float_value()),
            _ => todo!("Can't convert {bve:?} to type {t:?}"),
        }
    }
    pub fn as_bve(&self) -> BasicValueEnum<'a> {
        match self {
            Self::Int(var) | Self::UInt(var) => var.as_basic_value_enum(),
            Self::Float(var) => var.as_basic_value_enum(),
            Self::Pointer(var) => var.as_basic_value_enum(),
        }
    }
}
