use super::compile_variable::Variable;
use super::method_compiler::MethodCompiler;
use super::method_compiler::VirtualStack;
use crate::ir::op::{Op, OpKind};
use crate::ir::r#type::Type;
fn as_u64(i: i64) -> u64 {
    unsafe { std::mem::transmute(i) }
}
pub(crate) fn compile_arthm(
    compiler: &mut MethodCompiler,
    op: &Op,
    virt_stack: &mut VirtualStack,
) -> Option<()> {
    let (b, a) = virt_stack.pop().zip(virt_stack.pop()).unwrap();
    match op.kind() {
        OpKind::Add => virt_stack.push(compiler.add(a, b).unwrap()),
        OpKind::Sub => virt_stack.push(compiler.sub(a, b).unwrap()),
        OpKind::Mul => virt_stack.push(compiler.mul(a, b).unwrap()),
        OpKind::And => virt_stack.push(compiler.and(a, b).unwrap()),
        OpKind::Or => virt_stack.push(compiler.or(a, b).unwrap()),
        OpKind::XOr => virt_stack.push(compiler.xor(a, b).unwrap()),
        OpKind::Div => virt_stack.push(compiler.div(a, b).unwrap()),
        OpKind::SHL => virt_stack.push(compiler.shl(a, b).unwrap()),
        OpKind::Rem => virt_stack.push(compiler.rem(a, b).unwrap()),
        _ => panic!("INTERNAL LOGIC ERROR: compile_arthm recived non-arthemeic op!"),
    }
    Some(())
}
pub(crate) fn compile_op(
    compiler: &mut MethodCompiler,
    op: &Op,
    virt_stack: &mut VirtualStack,
) -> Option<()> {
    println!("Compiling op:{op:?}");
    match op.kind() {
        OpKind::Add
        | OpKind::And
        | OpKind::Or
        | OpKind::Div
        | OpKind::Mul
        | OpKind::Rem
        | OpKind::SHL
        | OpKind::XOr
        | OpKind::Sub => compile_arthm(compiler, op, virt_stack).unwrap(),
        OpKind::Not => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.not(a).unwrap());
        }
        OpKind::LDArg(arg_index) => {
            virt_stack.push(arg_index);
        }
        OpKind::LDCI32(val) => {
            virt_stack.push(compiler.add_const_i32(val));
        }
        OpKind::Ret => {
            if op.resolved_type().unwrap() == Type::Void {
                compiler.ret(None);
            } else {
                let ret = virt_stack.pop().unwrap();
                compiler.ret(Some(ret));
            }
        }
        OpKind::Neg => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.neg(a).unwrap());
        }
        OpKind::BGE(target)
        | OpKind::BLE(target)
        | OpKind::BEQ(target)
        | OpKind::BNE(target)
        | OpKind::BLT(target)
        | OpKind::BGT(target) => {
            let target_index = compiler.method().get_index_of_block_beginig_at(target);
            let target = compiler.block_at(target_index).unwrap();
            let (b, a) = virt_stack.pop().zip(virt_stack.pop()).unwrap();
            compiler.cj(
                a,
                b,
                target,
                compiler.get_next_block().unwrap(),
                &op.kind().cmp_type().expect(&format!(
                    "Cold not get the {:?}'s comparison type!",
                    op.kind()
                )),
            );
        }
        OpKind::BR(target) => {
            let target_index = compiler.method().get_index_of_block_beginig_at(target);
            let target = compiler.block_at(target_index).unwrap();
            compiler.unconditional_branch(target);
        }
        OpKind::STLoc(index) => {
            let a = virt_stack.pop().unwrap();
            compiler.set_local(a, index).unwrap();
        }
        OpKind::LDLoc(index) => {
            virt_stack.push(compiler.load_local(index).unwrap());
        }
        OpKind::Dup => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(a);
            virt_stack.push(a);
        }
        OpKind::Pop => {
            virt_stack.pop().unwrap();
        }
        OpKind::Nop => (),
        OpKind::ConvU8 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::U8).unwrap());
        }
        OpKind::ConvI8 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::I8).unwrap());
        }
        OpKind::ConvU16 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::U16).unwrap());
        }
        OpKind::ConvI16 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::I16).unwrap());
        }
        OpKind::ConvU32 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::U32).unwrap());
        }
        OpKind::ConvI32 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::I32).unwrap());
        }
        OpKind::ConvU64 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::U64).unwrap());
        }
        OpKind::ConvI64 => {
            let a = virt_stack.pop().unwrap();
            virt_stack.push(compiler.convert(a, Type::I64).unwrap());
        }
        _ => todo!("Unsuported OpKind:{:?}", op.kind()),
    }
    Some(())
}
