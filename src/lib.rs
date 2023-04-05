mod ir;
mod jit;
pub mod type_system;
#[doc(inline)]
pub use crate::ir::method::Method;
#[doc(inline)]
pub use crate::ir::op::OpKind;
#[doc(inline)]
pub use crate::ir::r#type::Type;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use ir::MethodIRError;
use jit::method_compiler::MethodCompiler;
//#[doc(inline)]
//pub use type_system::{AssemblyPrototype, CompiledAssembly};
fn opt_module(module: &Module) {
    use inkwell::passes::PassManager;
    let pass_manager: PassManager<Module> = PassManager::create(());
    pass_manager.add_instruction_combining_pass();
    pass_manager.add_instruction_simplify_pass();
    //pass_manager.add_correlated_value_propagation_pass();
    //pass_manager.add_basic_alias_analysis_pass();
    pass_manager.add_reassociate_pass();
    pass_manager.add_licm_pass();
    for i in 0..20 {
        println!("{}'th pass", i + 1);
        if !pass_manager.run_on(module) {
            break;
        }
    }
}
#[test]
fn test_nop() {
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::Void);
    let ops = [OpKind::Ret];
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `Nop`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("nop", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module
        .print_to_file("target/nop.lli")
        .expect("Could not write module to file!");
    module.verify().expect("Could not verify module!");
    opt_module(&module);
    module
        .print_to_file("target/opt_nop.lli")
        .expect("Could not write module to file!");
}
#[test]
fn test_add_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile  method  `Add`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("add_i32", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module.verify().expect("Could not verify module!");
    module
        .print_to_file("target/add_i32.lli")
        .expect("Could not write module to file!");
    opt_module(&module);
    module
        .print_to_file("target/opt_add_i32.lli")
        .expect("Could not write module to file!");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    unsafe {
        let f = execution_engine
            .get_function::<unsafe extern "C" fn(i32, i32) -> i32>("add_i32")
            .unwrap();
        assert!(f.call(5, 4) == 9);
    }
}
#[test]
fn test_wrong_return() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::F32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    if let Err(kind) = Method::from_ops(sig, &ops, &[]) {
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
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `Mag2`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("mag_2", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module.verify().expect("Could not verify module!");
    module
        .print_to_file("target/mag_2.lli")
        .expect("Could not write module to file!");
    opt_module(&module);
    module
        .print_to_file("target/opt_mag_2.lli")
        .expect("Could not write module to file!");
}
#[test]
fn test_abs() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDCI32(0), //1
        OpKind::BGE(6),    //2 -> |3,6|
        // a < 0
        OpKind::LDArg(0), //3
        OpKind::Neg,      //4
        OpKind::Ret,      //5
        // a > 0
        OpKind::LDArg(0), //6
        OpKind::Ret,      //7
    ];
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `Abs`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("abs", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module.verify().expect("Could not verify module!");
    module
        .print_to_file("target/abs.lli")
        .expect("Could not write module to file!");
    //opt_module(&module);
    module
        .print_to_file("target/opt_abs.lli")
        .expect("Could not write module to file!");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    unsafe {
        let f = execution_engine
            .get_function::<unsafe extern "C" fn(i32) -> i32>("abs")
            .unwrap();
        for i in -10..10 {
            println!("abs({i}) = {}", f.call(i));
        }
        assert_eq!(f.call(8), 8);
        assert_eq!(f.call(-8), 8);
    }
}
#[test]
fn test_factorial() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDCI32(1), //0
        OpKind::STLoc(0),  //1
        OpKind::LDCI32(1), //2
        OpKind::STLoc(1),  //3
        OpKind::BR(13),    //4
        //Loop body
        OpKind::LDLoc(0),  //5
        OpKind::LDLoc(1),  //6
        OpKind::Mul,       //7
        OpKind::STLoc(0),  //8
        OpKind::LDLoc(1),  // 9
        OpKind::LDCI32(1), //10
        OpKind::Add,       //11
        OpKind::STLoc(1),  //12
        //Loop head
        OpKind::LDLoc(1), //13
        OpKind::LDArg(0), //14
        OpKind::BLE(5),   //15
        //End loop
        OpKind::LDLoc(0), //15
        OpKind::Ret,      //16
    ];
    let method = Method::from_ops(sig, &ops, &[Type::I32, Type::I32])
        .expect("Could not compile method `Factorial`");
    let ctx = Context::create();
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("factorial", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module
        .print_to_file("target/factorial.lli")
        .expect("Could not write module to file!");
    module.verify().expect("Could not verify module!");
    opt_module(&module);
    module
        .print_to_file("target/opt_factorial.lli")
        .expect("Could not write module to file!");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    unsafe {
        let f = execution_engine
            .get_function::<unsafe extern "C" fn(i32) -> i32>("factorial")
            .unwrap();
        for i in 1..10 {
            println!("factorial({i}) = {}", f.call(i));
        }
        assert_eq!(f.call(1), 1, "Factorial 1");
        assert_eq!(f.call(2), 2, "Factorial 2");
        assert_eq!(f.call(3), 2 * 3, "Factorial 3");
        assert_eq!(f.call(4), 2 * 3 * 4, "Factorial 4");
        assert_eq!(f.call(5), 2 * 3 * 4 * 5, "Factorial 5");
        assert_eq!(f.call(6), 2 * 3 * 4 * 5 * 6, "Factorial 6");
    }
}
