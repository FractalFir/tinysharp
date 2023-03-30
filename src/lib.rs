mod ir;
use crate::ir::{method::Method,op::OpKind,r#type::Type};
use ir::*;
use inkwell::context::Context;
#[test]
fn test_nop() {
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::Void);
    let ops = [OpKind::Ret];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Nop`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.into_fn_type(&ctx);
    let mut fn_value = module.add_function("nop", fn_type, None);
    method.emmit_llvm(&mut fn_value,&ctx);
    module.verify().expect("Could not verify module!");
    module.print_to_file("target/nop.lli");
}
#[test]
fn test_add_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let method = Method::from_ops(sig, &ops).expect("Could not compile  method  `Add`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.into_fn_type(&ctx);
    let mut fn_value = module.add_function("add_i32", fn_type, None);
    method.emmit_llvm(&mut fn_value,&ctx);
    module.verify().expect("Could not verify module!");
    module.print_to_file("target/nop.lli");
}
#[test]
fn test_wrong_return() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::F32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    if let Err(kind) = Method::from_ops(sig, &ops) {
        if let MethodIRError::WrongReturnType { expected, got } = kind {
            assert_eq!(
                expected,
                Type::F32,
                "Error should notify that return type should have been F32"
            );
            assert_eq!(
                got,
                Type::I32,
                "Error should notify that return type was I32"
            );
        } else {
            panic!("Expected an return type error!");
        }
    } else {
        panic!("Expected an return type error!");
    }
}
#[test]
fn test_mag_2_f32() {
    let args: [Type; 3] = [Type::F32, Type::F32, Type::F32];
    let sig: (&[Type], Type) = (&args, Type::F32);
    let ops = [
        OpKind::LDArg(0),
        OpKind::LDArg(0),
        OpKind::Mul,
        OpKind::LDArg(1),
        OpKind::LDArg(1),
        OpKind::Mul,
        OpKind::Add,
        OpKind::LDArg(2),
        OpKind::LDArg(2),
        OpKind::Mul,
        OpKind::Add,
        OpKind::Ret,
    ];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Mag2`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.into_fn_type(&ctx);
    let mut fn_value = module.add_function("mag_2", fn_type, None);
    method.emmit_llvm(&mut fn_value,&ctx);
    module.verify().expect("Could not verify module!");
    module.print_to_file("target/nop.lli");
}
#[test]
fn test_abs() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),//0
        OpKind::LDCI32(0),//1
        OpKind::BGE(6),//2 -> |3,6|
        // a < 0
        OpKind::LDArg(0),//3
        OpKind::Neg,//4
        OpKind::Ret,//5
        // a > 0
        OpKind::LDArg(0),//6
        OpKind::Ret,//7
    ];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Abs`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.into_fn_type(&ctx);
    let mut fn_value = module.add_function("abs", fn_type, None);
    method.emmit_llvm(&mut fn_value,&ctx);
    module.verify().expect("Could not verify module!");
    module.print_to_file("target/nop.lli");
}
/*
#[test]
fn test_factorial(){
     use crate::ir_op::*;
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDCI32(1),
        OpKind::LDArg(0),
        OpKind::Mul,
        OpKind::LDArg(1),
        OpKind::LDArg(1),
        OpKind::Mul,
        OpKind::Add,
        OpKind::LDArg(2),
        OpKind::LDArg(2),
        OpKind::Mul,
        OpKind::Add,
        OpKind::Ret,
    ];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Mag2`");
}*/
