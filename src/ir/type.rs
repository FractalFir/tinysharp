use inkwell::context::Context;
use inkwell::types::AnyTypeEnum;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, VoidType};
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
    pub(crate) fn into_llvm_type<'a>(&self, ctx: &'a Context) -> AnyTypeEnum<'a> {
        match self {
            Type::Void => inkwell::types::AnyTypeEnum::VoidType(ctx.void_type()),
            Type::I32 => inkwell::types::AnyTypeEnum::IntType(ctx.i32_type()),
            Type::U32 => inkwell::types::AnyTypeEnum::IntType(ctx.i32_type()),
            Type::F32 => inkwell::types::AnyTypeEnum::FloatType(ctx.f32_type()),
            _ => todo!("Can't convert type {self:?} to llvm type!"),
        }
    }
    pub(crate) fn into_llvm_basic_type<'a>(&self, ctx: &'a Context) -> Option<BasicTypeEnum<'a>> {
        match self {
            Type::Void => None,
            Type::I32 => Some(BasicTypeEnum::IntType(ctx.i32_type())),
            Type::U32 => Some(BasicTypeEnum::IntType(ctx.i32_type())),
            Type::F32 => Some(BasicTypeEnum::FloatType(ctx.f32_type())),
            _ => todo!("Can't convert type {self:?} to llvm type!"),
        }
    }
}
