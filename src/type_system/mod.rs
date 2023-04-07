pub mod paths;
pub mod runtime;
use crate::ir::method::Method as IRMethod;
use crate::ir::r#type::{AsArgTypeList, GetType};
use crate::jit::{method_compiler::MethodCompiler, MethodCompileError};
use inkwell::{context::Context, module::Module, values::FunctionValue};
use paths::{ClassPath, MethodPath};
use runtime::Runtime;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
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
impl Drop for SingularRuntimeGuard {
    fn drop(&mut self) {
        println!("Droping: SRG!");
        unsafe {
            RUNTIME_COUNT.store(0, Ordering::Relaxed);
        }
    }
}
pub struct MethodRef<'rtime, Args: AsArgTypeList, Ret: GetType> {
    fptr: unsafe extern "C" fn(Args::RawType) -> Ret::RawType,
    _rtime: PhantomData<&'rtime ()>,
}
impl<'rtime, Args: AsArgTypeList, Ret: GetType> MethodRef<'rtime, Args, Ret> {
    pub unsafe fn get_ptr(&self) -> unsafe extern "C" fn(Args::RawType) -> Ret::RawType {
        self.fptr
    }
    pub fn call(&self, args: Args) -> Result<(), Ret> {
        todo!();
    }
}
struct Method<'a> {
    ir: IRMethod,
    fnc: FunctionValue<'a>,
    is_compiled: bool,
}
impl<'a> Method<'a> {
    fn new(ir: IRMethod, m: &Module<'a>, path: &MethodPath, ctx: &'a Context) -> Self {
        let fnc = m.add_function(path.ident(), ir.as_fn_type(ctx), None);
        Self {
            ir,
            fnc,
            is_compiled: false,
        }
    }
    fn compile(&mut self, ctx: &'a Context, module: &Module) -> Result<(), MethodCompileError> {
        if self.is_compiled {
            return Ok(());
        };
        MethodCompiler::new(ctx, self.fnc, &self.ir, module)?;
        Ok(())
    }
}
#[cfg(test)]
use crate::ir::Signature;
#[cfg(test)]
use crate::OpKind;
#[cfg(test)]
use crate::Type;
#[cfg(test)]
#[test]
fn add_method() {
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_sub = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(&sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[])
        .expect("Could not verify method `add`");
    let method_sub = Method::from_ops(Signature::new(&sig), &ops_sub, &[])
        .expect("Could not verify method `sub`");
    runtime.add_method(method_add, add_path);
    runtime.add_method(method_sub, sub_path);
}
#[cfg(test)]
#[test]
fn compile_methods() {
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_sub = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Sub, OpKind::Ret];
    let add_path = MethodPath::new(&"", &"Test", &"TestClass", &"Add", &Signature::new(&sig));
    let sub_path = MethodPath::new(&"", &"Test", &"TestClass", &"Sub", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[])
        .expect("Could not verify method `add`");
    let method_sub = Method::from_ops(Signature::new(&sig), &ops_sub, &[])
        .expect("Could not verify method `sub`");
    runtime.add_method(method_add, add_path);
    runtime.add_method(method_sub, sub_path);
    runtime.compile_all().expect("Could not compile methods!");
}
#[cfg(test)]
#[test]
fn method_lookup() {
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let add_path = MethodPath::new("", "Test", "TestClass", "Add", &Signature::new(&sig));
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_call_test = [
        OpKind::LDArg(0),
        OpKind::LDArg(0),
        OpKind::Call(add_path.clone(), Signature::new(&sig)),
        OpKind::LDArg(1),
        OpKind::Call(add_path.clone(), Signature::new(&sig)),
        OpKind::Ret,
    ];
    let call_test_path =
        MethodPath::new("", "Test", "TestClass", "CallTest", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[])
        .expect("Could not verify method `add`");
    let method_call_test = Method::from_ops(Signature::new(&sig), &ops_call_test, &[])
        .expect("Could not verify method `call_test`");
    runtime.add_method(method_add, add_path);
    runtime.add_method(method_call_test, call_test_path);
    runtime.compile_all().expect("Could not compile methods!");
    let call_test = runtime
        .get_method_ref::<(i32, i32), i32>("", "Test", "TestClass", "CallTest")
        .expect("Could not find method `call_test`");
}
#[cfg(test)]
#[test]
fn test_call() {
    use crate::ir::method::Method;
    let mut runtime = Runtime::init_await().expect("Could not initialise the runtime!");
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let add_path = MethodPath::new("", "Test", "TestClass", "Add", &Signature::new(&sig));
    let ops_add = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let ops_call_test = [
        OpKind::LDArg(0),
        OpKind::LDArg(0),
        OpKind::Call(add_path.clone(), Signature::new(&sig)),
        OpKind::LDArg(1),
        OpKind::Call(add_path.clone(), Signature::new(&sig)),
        OpKind::Ret,
    ];
    let call_test_path =
        MethodPath::new("", "Test", "TestClass", "CallTest", &Signature::new(&sig));
    let method_add = Method::from_ops(Signature::new(&sig), &ops_add, &[])
        .expect("Could not verify method `add`");
    let method_call_test = Method::from_ops(Signature::new(&sig), &ops_call_test, &[])
        .expect("Could not verify method `call_test`");
    runtime.add_method(method_add, add_path);
    runtime.add_method(method_call_test, call_test_path);
    runtime.compile_all().expect("Could not compile methods!");
    let call_test = runtime
        .get_method_ref::<(i32, i32), i32>("", "Test", "TestClass", "CallTest")
        .expect("Could not find method `call_test`");
}
