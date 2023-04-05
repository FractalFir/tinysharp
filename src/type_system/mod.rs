const IDENT_SPLIT: &str = "*";
use crate::ir::method::Method as IRMethod;
use crate::ir::Signature;
use std::pin::Pin;
use crate::jit::method_compiler::MethodCompiler;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::jit::MethodCompileError;
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
    fn compile(&mut self,ctx:&'a Context)->Result<MethodCompiler,MethodCompileError>{
        MethodCompiler::new(ctx,self.fnc,&self.ir)
    }
}
struct InnerRuntime<'a>{
    module: Module<'a>,
    methods: HashMap<MethodPath, Method<'a>>,
    ctx:&'a Context,
}
impl<'a> InnerRuntime<'a>{
    fn init(ctx:&'a Context)->Result<Self,RuntimeInitError>{
        let module = ctx.create_module("runtime");
        Ok(Self{ctx,module,methods:HashMap::new()})
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
            method.compile(&self.ctx)?;
        }
        Ok(())
    }
}
pub struct Runtime {
    ctx: Pin<Box<Context>>,
    //This is a hack. In reality, inner runtime only lives as long as `ctx` lives, but a struct can't hold a reference to its filed, so this is the only way to do it.
    runtime:Option<InnerRuntime<'static>>,
    guard:SingularRuntimeGuard,
}
impl Runtime{
    fn add_method(&mut self,method:IRMethod,path:MethodPath){
        self.runtime.as_mut().unwrap().add_method(method,path);
    }
    fn compile_all(&mut self)->Result<(),MethodCompileError>{
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
        let res = Self{ctx,guard,runtime};
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
pub struct ClassPath {
    identifier: String,
    assembly_end: usize,
    namespace_end: usize,
}
impl ClassPath {
    //pub fn finalize(prot:AssemblyPrototype)
    pub fn new(assembly: &str, namespace: &str, class_name: &str) -> Self {
        let assembly_end = assembly.len();
        let namespace_end = assembly_end + IDENT_SPLIT.len() + namespace.len();
        let mut identifier =
            String::with_capacity(namespace_end + IDENT_SPLIT.len() + class_name.len());
        identifier += assembly;
        identifier += IDENT_SPLIT;
        identifier += namespace;
        identifier += IDENT_SPLIT;
        identifier += class_name;
        ClassPath {
            identifier,
            assembly_end,
            namespace_end,
        }
    }
    pub fn assembly_name(&self) -> &str {
        &self.identifier[..self.assembly_end]
    }
    pub fn namespace(&self) -> &str {
        &self.identifier[(self.assembly_end + IDENT_SPLIT.len())..self.namespace_end]
    }
    pub fn class_name(&self) -> &str {
        &self.identifier[(self.namespace_end + IDENT_SPLIT.len())..]
    }
    pub(crate) fn ident(&self) -> &str {
        &self.identifier
    }
}
#[derive(Clone,Hash)]
pub struct MethodPath {
    identifier: String,
    assembly_end: usize,
    namespace_end: usize,
    class_end: usize,
    method_end: usize,
    //sig: Signature,
}
impl Eq for MethodPath{}
impl PartialEq for MethodPath{
    fn eq(&self,other:&Self)->bool{
       self.assembly_end == other.assembly_end && 
       self.namespace_end == other.namespace_end &&
       self.class_end == other.class_end &&
       self.method_end == other.method_end &&
       self.identifier == other.identifier
    }
}
impl MethodPath {
    fn new(
        assembly: &str,
        namespace: &str,
        class_name: &str,
        method_name: &str,
        sig: &Signature,
    ) -> Self {
        let assembly_end = assembly.len();
        let namespace_end = assembly_end + IDENT_SPLIT.len() + namespace.len();
        let class_end = namespace_end + IDENT_SPLIT.len() + class_name.len();
        let method_end = class_end + IDENT_SPLIT.len() + method_name.len();
        let sig_mangle = sig.to_mangle_string();
        let mut identifier =
            String::with_capacity(method_end + IDENT_SPLIT.len() + sig_mangle.len());
        identifier += assembly;
        identifier += IDENT_SPLIT;
        identifier += namespace;
        identifier += IDENT_SPLIT;
        identifier += class_name;
        identifier += IDENT_SPLIT;
        identifier += method_name;
        identifier += IDENT_SPLIT;
        identifier += &sig_mangle;
        MethodPath {
            identifier,
            assembly_end,
            namespace_end,
            class_end,
            method_end,
            //sig: sig.clone(),
        }
    }
    pub fn assembly_name(&self) -> &str {
        &self.identifier[..self.assembly_end]
    }
    pub fn namespace(&self) -> &str {
        &self.identifier[(self.assembly_end + IDENT_SPLIT.len())..self.namespace_end]
    }
    pub fn class_name(&self) -> &str {
        &self.identifier[(self.namespace_end + IDENT_SPLIT.len())..self.class_end]
    }
    pub fn method_name(&self) -> &str {
        &self.identifier[(self.class_end + IDENT_SPLIT.len())..self.method_end]
    }
    pub(crate) fn ident(&self) -> &str {
        &self.identifier
    }
}
#[cfg(test)]
fn rnd_name() -> String {
    const NAMES: [&str; 10] = [
        "Kiwi",
        "Apple",
        "Pear",
        "Banana",
        "Pineapple",
        "Blueberry",
        "Strawberry",
        "Peach",
        "Orange",
        "Targentine",
    ];
    let index = crate::ir::op_test::rnd_u32() as usize % NAMES.len();
    let index2 = crate::ir::op_test::rnd_u32() as usize % NAMES.len();
    NAMES[index].to_owned() + NAMES[index2]
}
#[cfg(test)]
#[test]
fn class_path() {
    for i in 0..1_000 {
        let assembly = rnd_name();
        let class = rnd_name();
        let namespace = rnd_name();
        let class_ref = ClassPath::new(&assembly, &namespace, &class);
        assert_eq!(assembly, class_ref.assembly_name());
        assert_eq!(class, class_ref.class_name());
        assert_eq!(namespace, class_ref.namespace());
        //println!("{}",class_ref.ident());
    }
}
#[cfg(test)]
#[test]
fn method_path() {
    use crate::ir::r#type::Type;
    let args: [Type; 8] = [
        Type::I64,
        Type::U64,
        Type::F64,
        Type::I32,
        Type::U32,
        Type::U16,
        Type::U8,
        Type::Bool,
    ];
    let sig: (&[Type], Type) = (&args, Type::I8);
    for i in 0..1_000 {
        let assembly = rnd_name();
        let class = rnd_name();
        let namespace = rnd_name();
        let method = rnd_name();
        let sig = Signature::new(sig);
        let class_ref = MethodPath::new(&assembly, &namespace, &class, &method, &sig);
        assert_eq!(assembly, class_ref.assembly_name());
        assert_eq!(class, class_ref.class_name());
        assert_eq!(namespace, class_ref.namespace());
        assert_eq!(method, class_ref.method_name());
        println!("{}", class_ref.ident());
    }
}
//Must be run in single thread mode!
#[test]#[ignore]
fn init_runtime(){
    {
        let runtime = Runtime::init().expect("Could not create runtime!");
    }
    let runtime_2 = Runtime::init().expect("Could not create runtime!");
    match Runtime::init(){
        Ok(_)=>panic!("Should not have been able to acquire another runtime!"),
        Err(_)=>(),
    }
    runtime_2;
}   //TODO:finalise types
use crate::OpKind;
use crate::Type;
#[cfg(test)]#[test]
fn add_method(){
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_sub = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(sig));
    let method_add = Method::from_ops(sig, &ops_add, &[]).expect("Could not verify method `add`");
    let method_sub = Method::from_ops(sig, &ops_sub, &[]).expect("Could not verify method `sub`");
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
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(sig));
    let method_add = Method::from_ops(sig, &ops_add, &[]).expect("Could not verify method `add`");
    let method_sub = Method::from_ops(sig, &ops_sub, &[]).expect("Could not verify method `sub`");
    runtime.add_method(method_add,add_path);
    runtime.add_method(method_sub,sub_path);
    runtime.compile_all().expect("Could not compile methods!");
}
