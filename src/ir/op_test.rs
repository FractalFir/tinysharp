use crate::{Method, OpKind, Type};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
pub(crate) fn rnd_u32() -> u32 {
    unsafe {
        static mut STATE_32: u32 = 1_750_234_402;
        STATE_32 ^= STATE_32 << 13;
        STATE_32 ^= STATE_32 >> 17;
        STATE_32 ^= STATE_32 << 5;
        STATE_32
    }
}
fn rnd_i32() -> i32 {
    unsafe { std::mem::transmute(rnd_u32()) }
}
fn rnd_i16() -> i16 {
    unsafe { std::mem::transmute((rnd_i32()%(i16::MAX as i32)) as i16)}
}
fn rnd_u16() -> u16 {
    (rnd_u32()%(u16::MAX as u32)) as u16
}
fn compile_fn<'a>(ctx: &'a Context, method: &Method) -> Module<'a> {
    use crate::MethodCompiler;
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(ctx);
    let fn_value = module.add_function("f", fn_type, None);
    let _mc = MethodCompiler::new(ctx, fn_value, method);
    match module.verify(){
        Ok(_)=>(),
        Err(msg)=>{
            let rnd = rnd_u32();
            let file = format!("target/test_res{rnd}.lli");
            module.print_to_file(&file);
            panic!("Module compilation failed with message:\n{msg}\n,dumping result to file:'{file}'");
        }
    }
    module
}
#[test]
fn add_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `add`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = rnd_i32() % (i32::MAX / 2);
        let rust_result = a + b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a + b");
    }
}
#[test]
fn sub_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = rnd_i32() % (i32::MAX / 2);
        let rust_result = a - b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a - b");
    }
}
#[test]
fn div_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Div, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = rnd_i32() % (i32::MAX / 2);
        let rust_result = a / b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a / b");
    }
}
#[test]
fn rem_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Rem, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = rnd_i32() % (i32::MAX / 2);
        let rust_result = a % b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a % b");
    }
}
#[test]
fn mul_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Mul, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % i32::from(i16::MAX);
        let b = rnd_i32() % i32::from(i16::MAX);
        let rust_result = a * b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a / b");
    }
}
#[test]
fn and_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::And, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let and =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
            .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % i32::from(i16::MAX);
        let b = rnd_i32() % i32::from(i16::MAX);
        let rust_result = a & b;
        let c = unsafe { and.call(a, b) };
        assert_eq!(rust_result, c, "a & b");
    }
}
#[test]
fn or_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Or, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % i32::from(i16::MAX);
        let b = rnd_i32() % i32::from(i16::MAX);
        let rust_result = a | b;
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a | b");
    }
}
#[test]
fn xor_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::XOr, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % i32::from(i16::MAX);
        let b = rnd_i32() % i32::from(i16::MAX);
        let rust_result = a ^ b;
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a ^ b");
    }
}
#[test]
fn not_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::Not, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `not`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32();
        let rust_result = !a;
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "!a");
    }
}
#[test]
fn dup_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::Dup, OpKind::Add, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = a + a;
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a + a");
    }
}
#[test]
fn pop_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),
        OpKind::LDCI32(3445),
        OpKind::Pop,
        OpKind::Ret,
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = a;
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a");
    }
}
#[test]
fn neg_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::Neg, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `neg`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = -a;
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "-a");
    }
}
#[test]
fn ld_st_loc_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),
        OpKind::STLoc(0),
        OpKind::LDLoc(0),
        OpKind::Ret,
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[Type::I32]).expect("Could not compile method `neg`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = a;
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a");
    }
}
#[test]
fn bge_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDCI32(0), //1
        OpKind::BGE(5),    //2
        OpKind::LDCI32(1), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(0), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    module
        .print_to_file("target/bge.lli")
        .expect("Could not write module to file!");
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = i32::from(a >= 0);
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a >= 0");
    }
    {
        let a = 0;
        let rust_result = i32::from(a >= 0);
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a < 0");
    }
}
#[test]
fn beq_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDArg(1),  //1
        OpKind::BEQ(5),    //2
        OpKind::LDCI32(0), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(1), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = if rnd_i32() >= 0 { rnd_i32() } else { a };
        let rust_result = i32::from(a == b);
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a == b");
    }
}
#[test]
fn bne_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDArg(1),  //1
        OpKind::BNE(5),    //2
        OpKind::LDCI32(0), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(1), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = if rnd_i32() >= 0 { rnd_i32() } else { a };
        let rust_result = i32::from(a != b);
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a != b");
    }
}
#[test]
fn blt_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDArg(1),  //1
        OpKind::BLT(5),    //2
        OpKind::LDCI32(0), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(1), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = if rnd_i32() % 2 == 0 { rnd_i32() } else { a };
        let rust_result = i32::from(a < b);
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a < b");
    }
}
#[test]
fn bgt_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDArg(1),  //1
        OpKind::BGT(5),    //2
        OpKind::LDCI32(0), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(1), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = if rnd_i32() % 2 == 0 { rnd_i32() } else { a };
        let rust_result = i32::from(a > b);
        let c = unsafe { f.call(a, b) };
        assert_eq!(rust_result, c, "a < b");
    }
}
#[test]
fn ble_i32() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  //0
        OpKind::LDCI32(0), //1
        OpKind::BLE(5),    //2
        OpKind::LDCI32(1), //3
        OpKind::Ret,       //4
        OpKind::LDCI32(0), //5
        OpKind::Ret,       //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `ble`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    module
        .print_to_file("target/bge.lli")
        .expect("Could not write module to file!");
    let f =
        unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>("f") }.unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let rust_result = i32::from(a <= 0);
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a <= 0");
    }
    {
        let a = 0;
        let rust_result = i32::from(a <= 0);
        let c = unsafe { f.call(a) };
        assert_eq!(rust_result, c, "a <= 0");
    }
}
#[test]
fn br() {
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::BR(3),     //0
        OpKind::LDCI32(0), //1
        OpKind::Ret,       //2
        OpKind::LDCI32(1), //3
        OpKind::Ret,       //4
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `add`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn() -> i32>("f") }.unwrap();
    unsafe {
        assert!(f.call() == 1);
    }
}
#[test]
fn nop() {
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::Void);
    let ops = [OpKind::Nop, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `nop`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn()>("f") }.unwrap();
    unsafe {
        f.call();
    }
}
#[test]
fn shl_i32() {
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::SHL, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `shl`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32() % (i32::MAX / 2);
        let b = rnd_i32().abs() % (30) + 1;
        let rust_result = a << b;
        let csharp_result = unsafe { f.call(a, b) };
        assert_eq!(rust_result, csharp_result, "a << b");
    }
}
#[test]
fn conv_i8() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I8);
    let ops = [OpKind::LDArg(0), OpKind::ConvI8, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_i8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i8>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32();
        fn i32_to_i8(a:i32)->i8{
            unsafe{std::mem::transmute(a.to_le_bytes()[0])}
        }
        let rust_result = i32_to_i8(a);
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as i8");
    }
}
#[test]
fn conv_u8() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::U8);
    let ops = [OpKind::LDArg(0), OpKind::ConvU8, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_u8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> u8>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32();
        fn i32_to_u8(a:i32)->u8{
            a.to_le_bytes()[0]
        }
        let rust_result = i32_to_u8(a);
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as u8");
    }
}
#[test]
fn conv_i16() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I16);
    let ops = [OpKind::LDArg(0), OpKind::ConvI16, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_i8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> i16>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32();
        fn i32_to_i16(a:i32)->i16{
            unsafe{std::mem::transmute(a.to_le_bytes()[0] as u16 + (a.to_le_bytes()[1] as u16)*256)}
        }
        let rust_result = i32_to_i16(a);
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as i8");
    }
}
#[test]
fn conv_u16() {
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::U16);
    let ops = [OpKind::LDArg(0), OpKind::ConvU16, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_u8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i32) -> u16>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i32();
        fn i32_to_u16(a:i32)->u16{
            a.to_le_bytes()[0] as u16 + (a.to_le_bytes()[1] as u16)*256
        }
        let rust_result = i32_to_u16(a);
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as u16");
    }
}
#[test]
fn conv_i32() {
    let args: [Type; 1] = [Type::I16];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::ConvI32, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_i8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i16) -> i32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i16();
        let rust_result = a as i32;
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as i32");
    }
}
#[test]
fn conv_u32() {
    let args: [Type; 1] = [Type::U16];
    let sig: (&[Type], Type) = (&args, Type::U32);
    let ops = [OpKind::LDArg(0), OpKind::ConvU32, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_u8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(u16) -> u32>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_u16();
        let rust_result = a as u32;
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as u16");
    }
}
#[test]
fn conv_i64() {
    let args: [Type; 1] = [Type::I16];
    let sig: (&[Type], Type) = (&args, Type::I64);
    let ops = [OpKind::LDArg(0), OpKind::ConvI64, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_i8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(i16) -> i64>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_i16();
        let rust_result = a as i64;
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as i64");
    }
}
#[test]
fn conv_u64() {
    let args: [Type; 1] = [Type::U16];
    let sig: (&[Type], Type) = (&args, Type::U64);
    let ops = [OpKind::LDArg(0), OpKind::ConvU64, OpKind::Ret];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `conv_u8`");
    let module = compile_fn(&ctx, &method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f = unsafe { execution_engine.get_function::<unsafe extern "C" fn(u16) -> u64>("f") }
        .unwrap();
    for _ in 0..10_000 {
        let a = rnd_u16();
        let rust_result = a as u64;
        let csharp_result = unsafe { f.call(a) };
        assert_eq!(rust_result, csharp_result, "a as u64");
    }
}
