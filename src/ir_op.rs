// use inkwhell::module::Module;
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
}
impl Type {
    fn arthm_promote(&self) -> Type {
        match self {
            Self::I64 | Self::U64 | Self::F64 | Self::I32 | Self::U32 | Self::F32 => *self,
            Self::I16 | Self::I8 => Self::I32,
            Self::U16 | Self::U8 => Self::U32,
            _ => todo!("Type promotion for arithmetic operations on type {self:?} unhanded!"),
        }
    }
}
type VType = Vec<Type>;
type VOp = Vec<Op>;
type VBlocks = Vec<OpBlock>;
#[derive(Clone, Copy, Debug)]
pub enum OpKind {
    Nop,
    Add,
    And,
    Ret,
    Mul,
    LDArg(usize),
}
impl OpKind {
    /// If instruction may branch, return it's target.
    fn branch_target(&self) -> Option<usize> {
        match self {
            Self::Nop | Self::Add | Self::Ret | Self::LDArg(_) | Self::Mul | Self::And => None,
        }
    }
}
fn get_op_type(a: Type, b: Type) -> Result<Type, MethodIRError> {
    let a = a.arthm_promote();
    let b = b.arthm_promote();
    if a != b {
        return Err(MethodIRError::OpOnMismatchedTypes(a, b));
    }
    Ok(a)
}
#[derive(Debug)]
struct Op {
    kind: OpKind,
    resolved_type: Option<Type>,
}
impl Op {
    fn from_kind(kind: OpKind) -> Self {
        Self {
            kind: kind,
            resolved_type: None,
        }
    }
    fn resolve(&mut self, state: &mut StackState, sig: &Signature) -> Result<(), MethodIRError> {
        match self.kind {
            OpKind::Ret => {
                let ret = if let Some(ret) = state.pop() {
                    ret
                } else {
                    Type::Void
                };
                if ret != sig.ret {
                    return Err(MethodIRError::WrongReturnType {
                        expected: sig.ret,
                        got: ret,
                    });
                }
                self.resolved_type = Some(ret);
            }
            OpKind::LDArg(arg) => {
                let t = sig.args[arg];
                self.resolved_type = Some(t);
                state.push(t);
            }
            OpKind::Add => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let sum = get_op_type(a, b)?;
                self.resolved_type = Some(sum);
                state.push(sum);
            }
            OpKind::Mul => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let mul = get_op_type(a, b)?;
                self.resolved_type = Some(mul);
                state.push(mul);
            }
            _ => todo!("OpKind {self:?} does not support resolving yet!"),
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
struct StackState {
    ///Innit state on beginning of block
    input: VType,
    ///Current state(output state at the end of block)
    output: VType,
}
impl StackState {
    pub fn push(&mut self, t: Type) {
        self.output.push(t);
    }
    pub fn pop(&mut self) -> Option<Type> {
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
enum BlockLink {
    Return,
}
#[derive(Debug)]
struct OpBlock {
    block: VOp,
    state_change: Option<StackState>,
    link_out: BlockLink,
}
impl OpBlock {
    fn resolve(&mut self, state: &mut StackState, sig: &Signature) -> Result<(), MethodIRError> {
        for mut op in &mut self.block {
            op.resolve(state, sig)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
struct Signature {
    args: VType,
    ret: Type,
}
impl Signature {
    fn new(src: SigType) -> Self {
        Self {
            args: src.0.into(),
            ret: src.1,
        }
    }
}
#[derive(Debug)]
pub struct Method {
    signature: Signature,
    blocks: VBlocks,
}
type SigType<'a> = (&'a [Type], Type);
fn spilt_into_blocks(ops: &[OpKind]) -> VBlocks {
    //nothing to do for now!
    let mut block = VOp::new();
    for op in ops {
        if let Some(target) = op.branch_target() {
            todo!("Branching not supported yet!");
        }
        block.push(Op::from_kind(*op));
    }
    vec![OpBlock {
        block,
        state_change: None,
        link_out: BlockLink::Return,
    }]
}
impl Method {
    fn resolve(&mut self) -> Result<(), MethodIRError> {
        if self.blocks.len() <= 1 {
            self.blocks[0].resolve(&mut StackState::default(), &self.signature)?;
        } else {
            todo!("Multiple Op Blocks unsupported!");
        }
        Ok(())
    }
    pub fn from_ops(sig: (&[Type], Type), ops: &[OpKind]) -> Result<Self, MethodIRError> {
        let blocks: VBlocks = spilt_into_blocks(ops);
        let mut res = Self {
            blocks,
            signature: Signature::new(sig),
        };
        res.resolve()?;
        println!("{res:?}");
        todo!();
        Ok(res)
    }
    /*
    pub fn emmit_llvm(&mut self,module:&mut Module){

    }*/
}
#[derive(Debug)]
pub enum MethodIRError {
    WrongReturnType { expected: Type, got: Type },
    OpOnMismatchedTypes(Type, Type),
}
