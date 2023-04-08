use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, IntType};
#[derive(Clone, Debug, PartialEq)]
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
    pub(crate) fn to_mangle_string(&self) -> String {
        match self {
            Self::I64 => "i64".to_owned(),
            Self::U64 => "u64".to_owned(),
            Self::F64 => "f64".to_owned(),
            Self::I32 => "i32".to_owned(),
            Self::U32 => "u32".to_owned(),
            Self::F32 => "f32".to_owned(),
            Self::I16 => "i16".to_owned(),
            Self::U16 => "u16".to_owned(),
            Self::I8 => "i8".to_owned(),
            Self::U8 => "u8".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Void => "void".to_owned(),
            _ => todo!("Can't create mangle string from type:{self:?}!"),
        }
    }
    pub(crate) fn as_int<'ctx>(&self, ctx: &'ctx Context) -> Option<IntType<'ctx>> {
        match self {
            Type::I64 | Type::U64 => Some(ctx.i64_type()),
            Type::I32 | Type::U32 => Some(ctx.i32_type()),
            Type::I16 | Type::U16 | Type::Char => Some(ctx.i16_type()),
            Type::I8 | Type::U8 => Some(ctx.i8_type()),

            _ => None,
        }
    }
    pub(crate) fn is_int(&self) -> bool {
        match self {
            Type::I64 | Type::U64 => true,
            Type::I32 | Type::U32 => true,
            Type::I16 | Type::U16 | Type::Char => true,
            Type::I8 | Type::U8 => true,
            _ => false,
        }
    }
    pub(crate) fn arthm_promote(&self) -> Type {
        match self {
            Self::I64 | Self::U64 | Self::F64 | Self::I32 | Self::U32 | Self::F32 => self.clone(),
            Self::I16 | Self::I8 => Self::I32,
            Self::U16 | Self::U8 => Self::U32,
            _ => todo!("Type promotion for arithmetic operations on type {self:?} unhanded!"),
        }
    }
    pub(crate) fn as_llvm_type<'ctx>(&self, ctx: &'ctx Context) -> AnyTypeEnum<'ctx> {
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
    pub(crate) fn as_llvm_basic_type<'ctx>(
        &self,
        ctx: &'ctx Context,
    ) -> Option<BasicTypeEnum<'ctx>> {
        match self {
            Type::Void => None,
            Type::I32 | Type::U32 => Some(BasicTypeEnum::IntType(ctx.i32_type())),
            Type::F32 => Some(BasicTypeEnum::FloatType(ctx.f32_type())),
            _ => todo!("Can't convert type {self:?} to llvm type!"),
        }
    }
    pub(crate) fn is_arthmetic(&self) -> bool {
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
pub trait GetType {
    type RawType;
    fn get_type() -> Type;
}
impl GetType for u8 {
    type RawType = u8;
    fn get_type() -> Type {
        Type::U8
    }
}
impl GetType for i8 {
    type RawType = i8;
    fn get_type() -> Type {
        Type::I8
    }
}
impl GetType for u16 {
    type RawType = u16;
    fn get_type() -> Type {
        Type::U16
    }
}
impl GetType for i16 {
    type RawType = i16;
    fn get_type() -> Type {
        Type::I16
    }
}
impl GetType for u32 {
    type RawType = u32;
    fn get_type() -> Type {
        Type::U32
    }
}
impl GetType for i32 {
    type RawType = i32;
    fn get_type() -> Type {
        Type::I32
    }
}
impl GetType for u64 {
    type RawType = u64;
    fn get_type() -> Type {
        Type::U64
    }
}
impl GetType for i64 {
    type RawType = i64;
    fn get_type() -> Type {
        Type::I64
    }
}
impl GetType for f32 {
    type RawType = f32;
    fn get_type() -> Type {
        Type::F32
    }
}
impl GetType for f64 {
    type RawType = f64;
    fn get_type() -> Type {
        Type::F64
    }
}
pub trait InteropSend: GetType {
    fn get_raw(&self) -> Self::RawType;
}
pub trait InteropRecive: GetType {
    fn get_converted(src: Self::RawType) -> Self;
}
impl<T: GetType<RawType = Self>> InteropSend for T
where
    T: Copy,
{
    fn get_raw(&self) -> Self {
        *self
    }
}
impl<T: GetType<RawType = Self>> InteropRecive for T
where
    T: Copy,
{
    fn get_converted(src: Self) -> Self {
        src
    }
}
trait TypeMarker {}
impl TypeMarker for Type {}
use std::borrow::Borrow;
pub trait AsArgTypeList {
    type Output: Borrow<[Type]>;
    type RawType;
    fn get_type_list() -> Self::Output;
}
impl AsArgTypeList for () {
    type Output = [Type; 0];
    type RawType = ();
    fn get_type_list() -> Self::Output {
        []
    }
}
impl<A: GetType> AsArgTypeList for (A,) {
    type Output = [Type; 1];
    type RawType = (A::RawType,);
    fn get_type_list() -> Self::Output {
        [A::get_type()]
    }
}
impl<A: GetType, B: GetType> AsArgTypeList for (A, B) {
    type Output = [Type; 2];
    type RawType = (A::RawType, B::RawType);
    fn get_type_list() -> Self::Output {
        [A::get_type(), B::get_type()]
    }
}
impl<A: GetType, B: GetType, C: GetType> AsArgTypeList for (A, B, C) {
    type Output = [Type; 3];
    type RawType = (A::RawType, B::RawType, C::RawType);
    fn get_type_list() -> Self::Output {
        [A::get_type(), B::get_type(), C::get_type()]
    }
}
pub trait ArgsToRaw {
    type Raw;
    fn to_raw(self) -> Self::Raw;
}
impl ArgsToRaw for () {
    type Raw = ();
    fn to_raw(self) -> () {}
}
impl<A: InteropSend> ArgsToRaw for (A,) {
    type Raw = (<A as GetType>::RawType,);
    fn to_raw(self) -> Self::Raw {
        (A::get_raw(&self.0),)
    }
}
impl<A: InteropSend, B: InteropSend> ArgsToRaw for (A, B) {
    type Raw = (<A as GetType>::RawType, <B as GetType>::RawType);
    fn to_raw(self) -> Self::Raw {
        (A::get_raw(&self.0), B::get_raw(&self.1))
    }
}
impl<A: InteropSend, B: InteropSend, C: InteropSend> ArgsToRaw for (A, B, C) {
    type Raw = (
        <A as GetType>::RawType,
        <B as GetType>::RawType,
        <C as GetType>::RawType,
    );
    fn to_raw(self) -> Self::Raw {
        (
            A::get_raw(&self.0),
            B::get_raw(&self.1),
            C::get_raw(&self.2),
        )
    }
}
//impl AsArgTypeList for (,)
