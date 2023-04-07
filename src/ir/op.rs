#![allow(dead_code)]
use super::r#type::Type;
use super::{ArgIndex, InstructionIndex, LocalVarIndex, MethodIRError, Signature, StackState};
use crate::jit::method_compiler::CMPType;
use crate::type_system::paths::{ClassPath, MethodPath};
#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms, clippy::module_name_repetitions)]
pub enum OpKind {
    Add,
    And,
    BGE(InstructionIndex), //Branch if greater or equal
    BLE(InstructionIndex), //Branch if less or equal
    BLT(InstructionIndex), //Branch if less than
    BGT(InstructionIndex), //Branch if less than
    BR(InstructionIndex),  //Unconditional branch.
    BEQ(InstructionIndex), //Branch if equal
    BNE(InstructionIndex), //Branch if not equal
    ConvU8,
    ConvI8,
    ConvU16,
    ConvI16,
    ConvU32,
    ConvI32,
    ConvU64,
    ConvI64,
    Call(MethodPath, Signature),
    Div,
    Dup,
    LDCI32(i32), //Load const i32
    LDArg(ArgIndex),
    LDNull,
    Nop,
    Not,
    Neg,
    Mul,
    Or,
    Pop,
    Ret,
    Rem,
    Sub,
    SHL,
    SHR,
    XOr,
    LDLoc(LocalVarIndex),
    STLoc(LocalVarIndex),
}
impl OpKind {
    pub(crate) fn cmp_type(&self) -> Option<CMPType> {
        match self {
            Self::BGE(_) => Some(CMPType::GE),
            Self::BLE(_) => Some(CMPType::LE),
            Self::BLT(_) => Some(CMPType::LT),
            Self::BGT(_) => Some(CMPType::GT),
            Self::BEQ(_) => Some(CMPType::EQ),
            Self::BNE(_) => Some(CMPType::NE),
            _ => None,
        }
    }
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
            | Self::Not
            | Self::Or
            | Self::Pop
            | Self::Rem
            | Self::Sub
            | Self::SHL
            | Self::SHR
            | Self::LDLoc(_)
            | Self::STLoc(_)
            | Self::ConvU8
            | Self::ConvI8
            | Self::ConvU16
            | Self::ConvI16
            | Self::ConvU32
            | Self::ConvI32
            | Self::ConvU64
            | Self::ConvI64
            | Self::Call(_, _)
            | Self::XOr => None,
            Self::BGE(target)
            | Self::BLE(target)
            | Self::BLT(target)
            | Self::BGT(target)
            | Self::BEQ(target)
            | Self::BNE(target)
            | Self::BR(target) => Some(*target),
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
#[derive(Clone, Debug)]
pub(crate) struct Op {
    kind: OpKind,
    resolved_type: Option<Type>,
}
impl Op {
    pub(crate) fn from_kind(kind: OpKind) -> Self {
        Self {
            kind,
            resolved_type: None,
        }
    }
    pub(crate) fn resolved_type(&self) -> Option<&Type> {
        self.resolved_type.as_ref()
    }
    pub(crate) fn kind(&self) -> &OpKind {
        &self.kind
    }
    pub(crate) fn resolve(
        &mut self,
        state: &mut StackState,
        sig: &Signature,
        locals: &[Type],
    ) -> Result<(), MethodIRError> {
        //println!("self:{self:?} State:{state:?}");
        match &self.kind {
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
                        expected: sig.ret.clone(),
                        got: ret.clone(),
                    });
                }
                self.resolved_type = Some(ret);
            }
            OpKind::LDArg(arg) => {
                let t = &sig.args[*arg];
                self.resolved_type = Some(t.clone());
                state.push(t.clone());
            }
            // Arthmentic
            OpKind::Mul | OpKind::Add | OpKind::Div | OpKind::Rem | OpKind::Sub => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res.clone());
                assert!(op_res.is_arthmetic());
                state.push(op_res);
            }
            // Bool-aplicable
            OpKind::And | OpKind::Or | OpKind::XOr | OpKind::SHL | OpKind::SHR => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res.clone());
                state.push(op_res);
            }
            OpKind::Not | OpKind::Neg => {
                let a = state.pop().unwrap();
                let op_res = a.arthm_promote();
                self.resolved_type = Some(op_res.clone());
                state.push(op_res);
            }
            OpKind::Dup => {
                let a = state.pop().unwrap();
                state.push(a.clone());
                state.push(a.clone());
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
            OpKind::BGE(_)
            | OpKind::BLE(_)
            | OpKind::BEQ(_)
            | OpKind::BNE(_)
            | OpKind::BLT(_)
            | OpKind::BGT(_) => {
                let a = state.pop().unwrap();
                let b = state.pop().unwrap();
                let op_res = get_op_type(a, b)?;
                self.resolved_type = Some(op_res);
            }
            OpKind::ConvU8 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::U8);
                state.push(Type::U8);
            }
            OpKind::ConvI8 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::I8);
                state.push(Type::I8);
            }
            OpKind::ConvU16 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::U16);
                state.push(Type::U16);
            }
            OpKind::ConvI16 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::I16);
                state.push(Type::I16);
            }
            OpKind::ConvU32 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::U32);
                state.push(Type::U32);
            }
            OpKind::ConvI32 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::I32);
                state.push(Type::I32);
            }
            OpKind::ConvU64 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::U64);
                state.push(Type::U64);
            }
            OpKind::ConvI64 => {
                let _ = state.pop().unwrap();
                self.resolved_type = Some(Type::I64);
                state.push(Type::I64);
            }
            OpKind::BR(_) => self.resolved_type = Some(Type::Void),
            OpKind::LDLoc(index) => {
                let loc_type = &locals[*index];
                state.push(loc_type.clone());
            }
            OpKind::STLoc(index) => {
                let s_type = state.pop().unwrap();
                if s_type != locals[*index] {
                    return Err(MethodIRError::LocalVarTypeMismatch(
                        s_type,
                        locals[*index].clone(),
                        *index,
                    ));
                }
            }
            OpKind::Call(_, sig) => {
                for arg_index in 0..sig.args.len() {
                    let arg = sig.args[sig.args.len() - arg_index - 1].clone();
                    let curr = state.pop().unwrap();
                    if arg != curr {
                        panic!("arg type mismatch in call!");
                    }
                }
                if sig.ret != Type::Void {
                    state.push(sig.ret.clone());
                }
            }
        }
        Ok(())
    }
}
