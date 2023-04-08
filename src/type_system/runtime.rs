use super::{
    paths::{ClassPath, MethodPath},
    Method, MethodCompileError, MethodRef, SingularRuntimeGuard,
};
use crate::ir::{
    method::Method as IRMethod,
    r#type::{AsArgTypeList, GetType, InteropRecive, InteropSend},
};
use crate::utilis::keyed_collection::KeyedCollection;
use core::marker::PhantomData;
use inkwell::{
    context::Context, execution_engine::ExecutionEngine, module::Module, OptimizationLevel,
};
use std::pin::Pin;

struct InnerRuntime<'a> {
    module: Module<'a>,
    methods: KeyedCollection<MethodPath, Method<'a>>,
    ctx: &'a Context,
    execution_engine: ExecutionEngine<'a>,
}
impl<'a> InnerRuntime<'a> {
    fn init(ctx: &'a Context) -> Result<Self, RuntimeInitError> {
        let module = ctx.create_module("runtime");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Default)
            .unwrap();
        Ok(Self {
            ctx,
            module,
            methods: KeyedCollection::new(),
            execution_engine,
        })
    }
    // Pretends `self` is static, offloading responsibility for ensuring proper lifetimes on the programmer.
    unsafe fn pretend_static(self) -> InnerRuntime<'static> {
        std::mem::transmute(self)
    }
}
impl<'a> InnerRuntime<'a> {
    fn add_method(&mut self, method: IRMethod, path: MethodPath) {
        let method = Method::new(method, &self.module, &path, self.ctx);
        println!("Inserting method mangled into:{}.", path.ident());
        self.methods.insert(path, method);
    }
    fn compile_all(&mut self) -> Result<(), MethodCompileError> {
        for method in self.methods.values_mut() {
            method.compile(&self.ctx, &self.module)?;
        }
        ExecutionEngine::link_in_mc_jit();
        Ok(())
    }
    fn verify(&self) -> Result<(), String> {
        match self.module.verify() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
    fn get_method_ptr<Args: AsArgTypeList, Ret: GetType>(
        &self,
        assembly: &str,
        namespace: &str,
        class_name: &str,
        method_name: &str,
    ) -> Option<unsafe extern "C" fn(Args::RawType) -> Ret::RawType> {
        use crate::Signature;
        let sig = Signature::from_types::<Args, Ret>();
        let path = MethodPath::new(assembly, namespace, class_name, method_name, &sig);
        println!("Searching for method mangled into:{}.", path.ident());
        unsafe {
            match self.execution_engine.get_function(path.ident()) {
                Ok(fptr) => Some(fptr.into_raw()),
                Err(err) => match err {
                    inkwell::execution_engine::FunctionLookupError::FunctionNotFound => None,
                    _ => panic!("Unhandled function lookup error:{err:?}"),
                },
            }
        }
    }
}
/// A container for the managed classes, methods and objects.
pub struct Runtime {
    /// This is not relay dead, and is classified as such due to use of unsafe.
    #[allow(dead_code)]
    ctx: Pin<Box<Context>>,
    //This is a hack. In reality, inner runtime only lives as long as `ctx` lives, but a struct can't hold a reference to its filed, so this is the only way to do it.
    runtime: Option<InnerRuntime<'static>>,
    _guard: SingularRuntimeGuard,
}
impl Runtime {
    /// Adds an method to the
    pub fn add_method(&mut self, method: IRMethod, path: MethodPath) {
        self.runtime.as_mut().unwrap().add_method(method, path);
    }
    /// Compiles all uncompiled methods
    pub fn compile_all(&mut self) -> Result<(), MethodCompileError> {
        self.runtime.as_mut().unwrap().compile_all()
    }
    pub fn verify(&self) -> Result<(), String> {
        self.runtime.as_ref().unwrap().verify()
    }
    pub fn get_method_ref<'a, Args: AsArgTypeList, Ret: GetType + InteropRecive>(
        &'a self,
        assembly: &str,
        namespace: &str,
        class_name: &str,
        method_name: &str,
    ) -> Option<MethodRef<'a, Args, Ret>> {
        let fptr = self.runtime.as_ref()?.get_method_ptr::<Args, Ret>(
            assembly,
            namespace,
            class_name,
            method_name,
        )?;
        Some(MethodRef {
            fptr,
            _rtime: PhantomData,
        })
    }
    pub fn load_asm<R: std::io::Read>(
        &mut self,
        asm: &mut R,
    ) -> Result<(), crate::importer::assembly::ImportError> {
        crate::importer::assembly::import_assembly(asm, self)
    }
}
#[derive(Debug)]
pub enum RuntimeInitError {
    RuntimeAlreadyPresent,
}
impl Runtime {
    /// Creates a new runtime with the default stdlib loaded.
    pub fn init() -> Result<Self, RuntimeInitError> {
        let guard = match SingularRuntimeGuard::acquire() {
            Some(guard) => guard,
            None => return Err(RuntimeInitError::RuntimeAlreadyPresent),
        };
        let ctx = Pin::new(Box::new(Context::create()));
        let runtime = InnerRuntime::init(&ctx)?;
        let runtime = Some(unsafe { InnerRuntime::pretend_static(runtime) });
        let res = Self {
            ctx,
            _guard: guard,
            runtime,
        };
        //Init
        Ok(res)
    }
    #[cfg(test)]
    pub fn init_await() -> Result<Self, RuntimeInitError> {
        let mut res = None;
        'wait: loop {
            match Self::init() {
                Err(err) => match err {
                    RuntimeInitError::RuntimeAlreadyPresent => (),
                    _ => return Err(err),
                },
                Ok(rtime) => {
                    res = Some(rtime);
                    break 'wait;
                }
            }
            let two_ms = std::time::Duration::from_micros(2);
            std::thread::sleep(two_ms);
        }
        Ok(res.unwrap())
    }
}
impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime = None;
    }
}
//Must be run in single thread mode!
#[test]
#[ignore]
fn init_runtime() {
    {
        let _runtime = Runtime::init().expect("Could not create runtime!");
    }
    let runtime_2 = Runtime::init().expect("Could not create runtime!");
    match Runtime::init() {
        Ok(_) => panic!("Should not have been able to acquire another runtime!"),
        Err(_) => (),
    }
    drop(runtime_2);
} //TODO:finalise types
#[test]
fn import() {
    use std::fs::File;
    let mut runtime = Runtime::init_await().expect("Coud not initialise the runtime!");
    let mut src =
        File::open("test_asm/SimpleFunctions/bin/Any CPU/Debug/net7.0/SimpleFunctions.dll")
            .expect("Could not open test file!");
    runtime
        .load_asm(&mut src)
        .expect("Could not load assembly!");
}
