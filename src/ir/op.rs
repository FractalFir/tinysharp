use super::r#type::Type;
use super::{ArgIndex, InstructionIndex, LocalVarIndex, MethodIRError, Signature, StackState};
use crate::Method;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::InstructionValue;
#[derive(Clone, Copy, Debug)]
pub enum OpKind {
    Add,
    And,
    BGE(InstructionIndex), //Branch if greater or equal
    BLE(InstructionIndex), //Branch if less or equal
    BR(InstructionIndex),  //Unconditional branch.
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
    LDLoc(LocalVarIndex),
    STLoc(LocalVarIndex),
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
            | Self::LDLoc(_)
            | Self::STLoc(_)
            | Self::XOr => None,
            Self::BGE(target) | Self::BLE(target) | Self::BR(target) => Some(*target),
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
        locals: &[Type],
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
            OpKind::BGE(_) | OpKind::BLE(_) => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res);
            }
            OpKind::BR(_) => self.resolved_type = Some(Type::Void),
            OpKind::LDLoc(index) => {
                let loc_type = locals[index];
                state.push(loc_type);
            }
            OpKind::STLoc(index) => {
                let s_type = state.pop().unwrap();
                if s_type != locals[index] {
                    return Err(MethodIRError::LocalVarTypeMismatch(
                        s_type,
                        locals[index],
                        index,
                    ));
                }
            }
        }
        Ok(())
    }
}
use inkwell::basic_block::BasicBlock;
use inkwell::values::AnyValueEnum;
