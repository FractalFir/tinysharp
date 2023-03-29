pub mod method;
pub mod op;
pub mod op_block;
pub mod r#type;
use op_block::OpBlock;
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
}
#[derive(Debug, Copy, Clone)]
pub(crate) enum BlockLink {
    Return,
    Branch(InstructionIndex, InstructionIndex),
}
