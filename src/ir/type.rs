use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum,IntType};
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
    pub(crate) fn to_mangle_string(&self)->String{
        match self{
            Self::I64=>"i64".to_owned(),
            Self::U64=>"u64".to_owned(),
            Self::F64=>"f64".to_owned(),
            Self::I32=>"i32".to_owned(),
            Self::U32=>"u32".to_owned(),
            Self::F32=>"f32".to_owned(),
            Self::I16=>"i16".to_owned(),
            Self::U16=>"u16".to_owned(),
            Self::I8=>"i8".to_owned(),
            Self::U8=>"u8".to_owned(),
            Self::Bool=>"bool".to_owned(),
            Self::Void=>"void".to_owned(),
            _=>todo!("Can't create mangle string from type:{self:?}!"),
        }
    }
    pub(crate) fn as_int(self,ctx: &'_ Context)->Option<IntType<'_>>{
        match self{
            Type::I64 | Type::U64 => Some(ctx.i64_type()),
            Type::I32 | Type::U32 => Some(ctx.i32_type()),
            Type::I16 | Type::U16 | Type::Char => Some(ctx.i16_type()),
            Type::I8 | Type::U8 => Some(ctx.i8_type()),
            
            _=>None,
        }
    }
    pub(crate) fn is_int(self)->bool{
        match self{
            Type::I64 | Type::U64 => true,
            Type::I32 | Type::U32 => true,
            Type::I16 | Type::U16 | Type::Char => true,
            Type::I8 | Type::U8 => true,
            _=>false,
        }
    }
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
            Type::I64 | Type::U64 => inkwell::types::AnyTypeEnum::IntType(ctx.i64_type()),
            Type::I32 | Type::U32 => inkwell::types::AnyTypeEnum::IntType(ctx.i32_type()),
            Type::I16 | Type::U16 => inkwell::types::AnyTypeEnum::IntType(ctx.i16_type()),
            Type::I8 | Type::U8 => inkwell::types::AnyTypeEnum::IntType(ctx.i8_type()),
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
        matches!(
            self,
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
                | Self::Char
        )
    }
}
