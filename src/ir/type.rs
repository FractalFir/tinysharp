use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Type {
    I64,
    U64,
    F64,
    I32,
    U32,
    F32,
    I16,
    U16,
    U8,
    I8,
    UPtr,
    IPtr,
    Char,
    Void,
    ObjRef,
    Bool,
}
impl Type {
    pub(crate) fn arthm_promote(self) -> Type {
        match self {
            Self::I64 | Self::U64 | Self::F64 | Self::I32 | Self::U32 | Self::F32 => self,
            Self::I16 | Self::I8 => Self::I32,
            Self::U16 | Self::U8 => Self::U32,
            _ => todo!("Type promotion for arithmetic operations on type {self:?} unhanded!"),
        }
    }
    pub(crate) fn as_llvm_type(self, ctx: &'_ Context) -> AnyTypeEnum<'_> {
        match self {
            Type::Void => inkwell::types::AnyTypeEnum::VoidType(ctx.void_type()),
            Type::I32 | Type::U32 => inkwell::types::AnyTypeEnum::IntType(ctx.i32_type()),
            Type::F32 => inkwell::types::AnyTypeEnum::FloatType(ctx.f32_type()),
            _ => todo!("Can't convert type {self:?} to llvm type!"),
        }
    }
    pub(crate) fn as_llvm_basic_type(self, ctx: &'_ Context) -> Option<BasicTypeEnum<'_>> {
        match self {
            Type::Void => None,
            Type::I32 | Type::U32 => Some(BasicTypeEnum::IntType(ctx.i32_type())),
            Type::F32 => Some(BasicTypeEnum::FloatType(ctx.f32_type())),
            _ => todo!("Can't convert type {self:?} to llvm type!"),
        }
    }
    pub(crate) fn is_arthmetic(self) -> bool {
        matches!(self,
            Self::I64
            | Self::U64
            | Self::F64
            | Self::I32
            | Self::U32
            | Self::F32
            | Self::I16
            | Self::U16
            | Self::U8
            | Self::I8
            | Self::UPtr
            | Self::IPtr
            | Self::Char)
    }
}
