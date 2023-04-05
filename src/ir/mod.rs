pub mod method;
pub mod op;
pub mod op_block;
pub mod r#type;
use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use op::OpKind;
use op_block::OpBlock;
use r#type::Type;
#[derive(Debug)]
pub enum MethodIRError {
    WrongReturnType { expected: Type, got: Type },
    OpOnMismatchedTypes(Type, Type),
    LocalVarTypeMismatch(Type, Type, usize),
    StateUnresolvedNoError,
}
pub type VType = Vec<Type>;
pub type SigType<'a> = (&'a [Type], Type);
pub(crate) type VOp = Vec<op::Op>;
pub(crate) type VBlocks = Vec<OpBlock>;
pub type ArgIndex = usize;
pub type InstructionIndex = usize;
pub type LocalVarIndex = usize;
#[derive(Debug, Clone, Default)]
pub(crate) struct StackState {
    ///Innit state on beginning of block
    // input: VType,
    ///Current state(output state at the end of block)
    output: VType,
}
impl StackState {
    pub(crate) fn push(&mut self, t: Type) {
        self.output.push(t);
    }
    pub(crate) fn pop(&mut self) -> Option<Type> {
        self.output.pop()
    }
    /*pub(crate) fn is_empty(&mut self) -> bool {
        self.output.is_empty()
    }*/
}
#[derive(Debug, Clone)]
pub(crate) struct Signature {
    args: VType,
    ret: Type,
}
impl Signature {
    pub(crate) fn to_mangle_string(&self) -> String {
        let mut res = String::new();
        for arg in &self.args {
            res += &arg.to_mangle_string();
            res += "/";
        }
        res
    }
    pub(crate) fn new(src: SigType) -> Self {
        Self {
            args: src.0.into(),
            ret: src.1,
        }
    }
    pub(crate) fn argc(&self) -> usize {
        self.args.len()
    }
    pub(crate) fn args(&self) -> &[Type] {
        &self.args
    }
    pub(crate) fn as_fn_type<'a>(&self, ctx: &'a Context) -> FunctionType<'a> {
        let mut args = Vec::new();
        for arg in &self.args {
            let t: BasicMetadataTypeEnum = arg
                .as_llvm_type(ctx)
                .try_into()
                .expect("Type can't be a function parameter!");
            args.push(t);
        }
        let args: &[BasicMetadataTypeEnum] = &args;
        let ret_type = self.ret.as_llvm_type(ctx);
        if let Ok(ret_type) = ret_type.try_into() {
            let ret_type: BasicTypeEnum = ret_type;
            ret_type.fn_type(args, false)
        } else if let inkwell::types::AnyTypeEnum::VoidType(ret_type) = ret_type {
            ret_type.fn_type(args, false)
        } else {
            panic!("A LLVM function can't return {ret_type:?}!");
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub(crate) enum BlockLink {
    Return,
    Branch(InstructionIndex, InstructionIndex),
    Pass, //Passes to the next instruction normaly
}
#[cfg(test)]
pub(crate) mod op_test;
