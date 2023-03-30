use super::r#type::Type;
use super::{ArgIndex, InstructionIndex, MethodIRError, Signature, StackState};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::InstructionValue;
#[derive(Clone, Copy, Debug)]
pub enum OpKind {
    Add,
    And,
    BGE(InstructionIndex), //Branch if greater or equal
    Div,
    Dup,
    LDCI32(i32), //Load const i32
    LDArg(ArgIndex),
    LDNull,
    Nop,
    Neg,
    Mul,
    Or,
    Pop,
    Ret,
    Rem,
    Sub,
    XOr,
}
impl OpKind {
    /// If instruction may branch, return it's target.
    pub(crate) fn branch_target(&self) -> Option<InstructionIndex> {
        match self {
            Self::Nop
            | Self::Add
            | Self::Ret
            | Self::LDArg(_)
            | Self::Mul
            | Self::And
            | Self::Div
            | Self::Dup
            | Self::LDCI32(_)
            | Self::LDNull
            | Self::Neg
            | Self::Or
            | Self::Pop
            | Self::Rem
            | Self::Sub
            | Self::XOr => None,
            Self::BGE(target) => Some(*target),
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
#[derive(Copy, Clone, Debug)]
pub(crate) struct Op {
    kind: OpKind,
    resolved_type: Option<Type>,
}
impl Op {
    pub(crate) fn from_kind(kind: OpKind) -> Self {
        Self {
            kind: kind,
            resolved_type: None,
        }
    }
    pub(crate) fn resolved_type(&self) -> Option<Type> {
        self.resolved_type
    }
    pub(crate) fn kind(&self) -> OpKind {
        self.kind
    }
    pub(crate) fn resolve(
        &mut self,
        state: &mut StackState,
        sig: &Signature,
    ) -> Result<(), MethodIRError> {
        match self.kind {
            OpKind::Nop => {
                //TODO:Reconsider resolving type being present for all ops. Seems kinda stupid now.
                self.resolved_type = Some(Type::Void);
            }
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
            // Arthmentic
            OpKind::Mul | OpKind::Add | OpKind::Div | OpKind::Rem | OpKind::Sub => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res);
                //TODO: op_res.is_arithmetic()
                state.push(op_res);
            }
            // Bool-aplicable
            OpKind::And | OpKind::Or | OpKind::XOr => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res);
                state.push(op_res);
            }
            OpKind::Dup => {
                let a = state.pop().unwrap();
                state.push(a);
                state.push(a);
                self.resolved_type = Some(a);
            }
            OpKind::Pop => {
                let t = state.pop().unwrap();
                self.resolved_type = Some(t);
            }
            OpKind::LDCI32(_) => {
                self.resolved_type = Some(Type::I32);
                state.push(Type::I32);
            }
            OpKind::LDNull => {
                self.resolved_type = Some(Type::ObjRef);
                state.push(Type::ObjRef);
            }
            OpKind::Neg => {
                let a = state.pop().unwrap();
                let a = a.arthm_promote();
                self.resolved_type = Some(a);
                state.push(a);
            } //_ => todo!("OpKind {self:?} does not support resolving yet!"),
            OpKind::BGE(_) => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res);
            }
        }
        Ok(())
    }
}
use inkwell::basic_block::BasicBlock;
use inkwell::values::AnyValueEnum;
