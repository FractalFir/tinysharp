mod paths;
pub use paths::*;
use crate::ir::method::Method as IRMethod;
use std::pin::Pin;
use crate::jit::method_compiler::MethodCompiler;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::jit::MethodCompileError;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::OptimizationLevel;
static mut RUNTIME_COUNT: AtomicUsize = AtomicUsize::new(0);
//Ensures only one runtime exists at any given time.
struct SingularRuntimeGuard();
impl SingularRuntimeGuard {
    fn acquire() -> Option<Self> {
        unsafe {
            match RUNTIME_COUNT.compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => Some(Self()),
                Err(_) => None,
            }
        }
    }
}
impl Drop for SingularRuntimeGuard{
    fn drop(&mut self){
        println!("Droping: SRG!");
        unsafe{
             RUNTIME_COUNT.store(0,Ordering::Relaxed);
        }
    }
}
struct Method<'a> {
    ir: IRMethod,
    fnc: FunctionValue<'a>,
    is_compiled: bool,
}
impl<'a> Method<'a>{
    fn new(ir:IRMethod,m:&Module<'a>,path:&MethodPath,ctx:&'a Context)->Self{
        let fnc = m.add_function(path.ident(), ir.as_fn_type(ctx), None);
        Self{ir,fnc,is_compiled:false}
    }
    fn compile(&mut self,ctx:&'a Context,module:&Module)->Result<(),MethodCompileError>{
        if self.is_compiled{return Ok(())}; 
        MethodCompiler::new(ctx,self.fnc,&self.ir,module)?;
        Ok(())
    }
}
struct InnerRuntime<'a>{
    module: Module<'a>,
    methods: HashMap<MethodPath, Method<'a>>,
    ctx:&'a Context,
    execution_engine:ExecutionEngine<'a>,
}
impl<'a> InnerRuntime<'a>{
    fn init(ctx:&'a Context)->Result<Self,RuntimeInitError>{
        let module = ctx.create_module("runtime");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Default).unwrap();
        Ok(Self{ctx,module,methods:HashMap::new(),execution_engine})
    }
    // Pretends `self` is static, offloading responsibility for ensuring proper lifetimes on the programmer.
    unsafe fn pretend_static(self)->InnerRuntime<'static>{
        std::mem::transmute(self)
    }
}
impl<'a> InnerRuntime<'a>{
    fn add_method(&mut self,method:IRMethod,path:MethodPath){
        let method = Method::new(method,&self.module,&path,self.ctx);
        self.methods.insert(path,method);
    }
    fn compile_all(&mut self)->Result<(),MethodCompileError>{
       for method in self.methods.values_mut(){
            method.compile(&self.ctx,&self.module)?;
        }
        Ok(())
    }
}
/// A container for the managed classes, methods and objects.
pub struct Runtime {
    /// This is not relay dead, and is classified as such due to use of unsafe.
    #[allow(dead_code)]
    ctx: Pin<Box<Context>>,
    //This is a hack. In reality, inner runtime only lives as long as `ctx` lives, but a struct can't hold a reference to its filed, so this is the only way to do it.
    runtime:Option<InnerRuntime<'static>>,
    _guard:SingularRuntimeGuard,
}
impl Runtime{
    /// Adds an method to the 
    pub fn add_method(&mut self,method:IRMethod,path:MethodPath){
        self.runtime.as_mut().unwrap().add_method(method,path);
    }
    /// Compiles
    pub fn compile_all(&mut self)->Result<(),MethodCompileError>{
        self.runtime.as_mut().unwrap().compile_all()
    }
}
#[derive(Debug)]
pub enum RuntimeInitError{
    RuntimeAlreadyPresent,
}
impl Runtime {
    /// Creates a new runtime with the default stdlib loaded. 
    pub fn init()->Result<Self,RuntimeInitError>{
        let guard = match SingularRuntimeGuard::acquire(){
            Some(guard)=>guard,
            None=>return Err(RuntimeInitError::RuntimeAlreadyPresent),
        };
        let ctx = Pin::new(Box::new(Context::create()));
        let runtime = InnerRuntime::init(&ctx)?;
        let runtime = Some(unsafe{InnerRuntime::pretend_static(runtime)});
        let res = Self{ctx,_guard:guard,runtime};
        //Init
        Ok(res)
    }
    #[cfg(test)]
    fn init_await()->Result<Self,RuntimeInitError>{
        let mut res = None;
        'wait: loop {
            match Self::init(){
                Err(err)=>match err{
                    RuntimeInitError::RuntimeAlreadyPresent=>(),
                    _=>return Err(err),
                },
                Ok(rtime)=>{
                    res = Some(rtime);
                    break 'wait;
                },
            }
            let two_ms = std::time::Duration::from_micros(2);
            std::thread::sleep(two_ms);
        }
        Ok(res.unwrap())
    }
}
impl Drop for Runtime{
    fn drop(&mut self){
        self.runtime = None;
    }
}
//Must be run in single thread mode!
#[test]#[ignore]
fn init_runtime(){
    {
        let _runtime = Runtime::init().expect("Could not create runtime!");
    }
    let runtime_2 = Runtime::init().expect("Could not create runtime!");
    match Runtime::init(){
        Ok(_)=>panic!("Should not have been able to acquire another runtime!"),
        Err(_)=>(),
    }
    drop(runtime_2);
}   //TODO:finalise types
#[cfg(test)]
use crate::OpKind;
#[cfg(test)]
use crate::Type;
#[cfg(test)]
use crate::ir::Signature;
#[cfg(test)]#[test]
fn add_method(){
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_sub = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(&sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[]).expect("Could not verify method `add`");
    let method_sub = Method::from_ops(Signature::new(&sig), &ops_sub, &[]).expect("Could not verify method `sub`");
    runtime.add_method(method_add,add_path);
    runtime.add_method(method_sub,sub_path);
}
#[cfg(test)]#[test]
fn compile_methods(){
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_sub = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(&sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[]).expect("Could not verify method `add`");
    let method_sub = Method::from_ops(Signature::new(&sig), &ops_sub, &[]).expect("Could not verify method `sub`");
    runtime.add_method(method_add,add_path);
    runtime.add_method(method_sub,sub_path);
    runtime.compile_all().expect("Could not compile methods!");
}
#[cfg(test)]#[test]
fn test_call(){
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let add_path = MethodPath::new("", "Test", "TestClass", "Add", &Signature::new(&sig));
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_call_test = [OpKind::LDArg(0), OpKind::LDArg(0), OpKind::Call(add_path.clone(),Signature::new(&sig)), OpKind::LDArg(1),OpKind::Call(add_path.clone(),Signature::new(&sig)), OpKind::Ret];
    let call_test_path = MethodPath::new("", "Test", "TestClass", "CallTest", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[]).expect("Could not verify method `add`");
    let method_call_test = Method::from_ops(Signature::new(&sig), &ops_call_test, &[]).expect("Could not verify method `sub`");
    runtime.add_method(method_add,add_path);
    runtime.add_method(method_call_test,call_test_path);
    runtime.compile_all().expect("Could not compile methods!");
}
