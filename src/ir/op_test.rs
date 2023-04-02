use inkwell::context::Context;
use inkwell::module::Module;
use crate::{Type,OpKind,Method};
use inkwell::OptimizationLevel;
fn test_rnd_i32()->i32{
    static mut I32_VAL:i32 = 1193240232;
    unsafe{
    I32_VAL ^= 328429394;
    I32_VAL <<= 1;
    if I32_VAL.abs() < 123245{
        I32_VAL += 20924;
    }
    I32_VAL}
}
fn compile_fn<'a>(ctx:&'a Context,method:&Method)->Module<'a>{
    use crate::MethodCompiler;
    let module = ctx.create_module("my_mod");
    let fn_type = method.as_fn_type(&ctx);
    let fn_value = module.add_function("f", fn_type, None);
    let _mc = MethodCompiler::new(&ctx, fn_value, &method);
    module.verify().expect("Could not verify module!");
    module
}
#[test]
fn add_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Add,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `add`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let b = test_rnd_i32()%(i32::MAX/2);
        let radd = a+b;
        let cadd = unsafe{f.call(a,b)};
        assert_eq!(radd,cadd,"a + b");
    } 
}
#[test]
fn sub_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Sub,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let b = test_rnd_i32()%(i32::MAX/2);
        let radd = a - b;
        let cadd = unsafe{f.call(a,b)};
        assert_eq!(radd,cadd,"a - b");
    } 
}
#[test]
fn div_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Div,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let b = test_rnd_i32()%(i32::MAX/2);
        let radd = a / b;
        let cadd = unsafe{f.call(a,b)};
        assert_eq!(radd,cadd,"a / b");
    } 
}
#[test]
fn rem_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Rem,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let b = test_rnd_i32()%(i32::MAX/2);
        let radd = a % b;
        let cadd = unsafe{f.call(a,b)};
        assert_eq!(radd,cadd,"a % b");
    } 
}
#[test]
fn mul_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Mul,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i16::MAX as i32);
        let b = test_rnd_i32()%(i16::MAX as i32);
        let radd = a * b;
        let cadd = unsafe{f.call(a,b)};
        assert_eq!(radd,cadd,"a / b");
    } 
}
#[test]
fn and_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::And,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i16::MAX as i32);
        let b = test_rnd_i32()%(i16::MAX as i32);
        let r = a & b;
        let c = unsafe{f.call(a,b)};
        assert_eq!(r,c,"a & b");
    } 
}
#[test]
fn or_i32(){
    let args: [Type; 2] = [Type::I32,Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::LDArg(1), 
        OpKind::Or,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32,i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i16::MAX as i32);
        let b = test_rnd_i32()%(i16::MAX as i32);
        let r = a | b;
        let c = unsafe{f.call(a,b)};
        assert_eq!(r,c,"a | b");
    } 
}
#[test]
fn not_i32(){
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),  
        OpKind::Not,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `not`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32();
        let r = !a;
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"!a");
    } 
}
#[test]
fn dup_i32(){
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),
        OpKind::Dup,
        OpKind::Add,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `sub`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%((i32::MAX as i32)/2);
        let r = a + a;
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a + a");
    } 
}
#[test]
fn pop_i32(){
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
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%((i32::MAX as i32)/2);
        let r = a;
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a");
    } 
}
#[test]
fn neg_i32(){
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0), 
        OpKind::Neg,     
        OpKind::Ret,      
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `neg`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let r = -a;
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"-a");
    } 
}
#[test]
fn ld_st_loc_i32(){
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
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let r = a;
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a");
    } 
}
#[test]
fn bge_i32(){
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),   //0
        OpKind::LDCI32(0),  //1
        OpKind::BGE(5),     //2
        
        OpKind::LDCI32(1),  //3
        OpKind::Ret,        //4
        
        OpKind::LDCI32(0),  //5
        OpKind::Ret,        //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `bge`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    module
        .print_to_file("target/bge.lli")
        .expect("Could not write module to file!");
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let r = if a >= 0 {1} else {0};
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a >= 0");
    } 
    {
        let a = 0;
        let r = if a >= 0 {1} else {0};
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a < 0");
    }
   
}
#[test]
fn ble_i32(){
    let args: [Type; 1] = [Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [
        OpKind::LDArg(0),   //0
        OpKind::LDCI32(0),  //1
        OpKind::BLE(5),     //2
        
        OpKind::LDCI32(1),  //3
        OpKind::Ret,        //4
        
        OpKind::LDCI32(0),  //5
        OpKind::Ret,        //6
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `ble`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    module
        .print_to_file("target/bge.lli")
        .expect("Could not write module to file!");
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn(i32) -> i32>("f")}
        .unwrap();
    for _ in 0..10_000{
        let a = test_rnd_i32()%(i32::MAX/2);
        let r = if a <= 0 {1} else {0};
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a <= 0");
    } 
    {
        let a = 0;
        let r = if a <= 0 {1} else {0};
        let c = unsafe{f.call(a)};
        assert_eq!(r,c,"a <= 0");
    }
}
#[test]
fn br(){
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [    
        OpKind::BR(3),//0
        OpKind::LDCI32(0),//1
        OpKind::Ret,  //2
        OpKind::LDCI32(1),//3
        OpKind::Ret,//4
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `add`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn() -> i32>("f")}
        .unwrap();
    unsafe{
        assert!(f.call() == 1);
    }
}  
#[test]
fn nop(){
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::Void);
    let ops = [    
        OpKind::Nop,
        OpKind::Ret,
    ];
    let ctx = Context::create();
    let method = Method::from_ops(sig, &ops, &[]).expect("Could not compile method `nop`");
    let module = compile_fn(&ctx,&method);
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();
    let f =  unsafe {execution_engine
        .get_function::<unsafe extern "C" fn()>("f")}
        .unwrap();
    unsafe{
        f.call();
    }
}  

