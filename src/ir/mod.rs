pub mod method;
pub mod op;
pub mod op_block;
pub mod r#type;
pub(crate) mod method_compiler;
use inkwell::context::Context;
use inkwell::types::BasicType;
use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
pub(crate) use op_block::OpBlock;
pub(crate) use op::OpKind;
use r#type::Type;
#[derive(Debug)]
pub enum MethodIRError {
    WrongReturnType { expected: Type, got: Type },
    OpOnMismatchedTypes(Type, Type),
}
pub type VType = Vec<Type>;
pub type SigType<'a> = (&'a [Type], Type);
pub(crate) type VOp = Vec<op::Op>;
pub(crate) type VBlocks = Vec<OpBlock>;
pub type ArgIndex = usize;
pub type InstructionIndex = usize;
#[derive(Debug, Clone)]
pub(crate) struct StackState {
    ///Innit state on beginning of block
    input: VType,
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
    pub(crate) fn is_empty(&mut self) -> bool {
        self.output.is_empty()
    }
}
impl Default for StackState {
    fn default() -> Self {
        Self {
            input: VType::new(),
            output: VType::new(),
        }
    }
}
#[derive(Debug)]
pub(crate) struct Signature {
    args: VType,
    ret: Type,
}
impl Signature {
    pub(crate) fn new(src: SigType) -> Self {
        Self {
            args: src.0.into(),
            ret: src.1,
        }
    }
    pub(crate) fn into_fn_type<'a>(&self, ctx: &'a Context) -> FunctionType<'a> {
        let mut args = Vec::new();
        for arg in &self.args {
            let t: BasicMetadataTypeEnum = arg
                .into_llvm_type(ctx)
                .try_into()
                .expect("Type can't be a function parameter!");
            args.push(t);
        }
        use inkwell::types::BasicMetadataTypeEnum;
        let args: &[BasicMetadataTypeEnum] = &args;
        let ret_type = self.ret.into_llvm_type(ctx);
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
