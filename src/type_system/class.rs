use super::{paths::ClassPath, Method};
use crate::utilis::keyed_collection::KCRef;
use inkwell::types::StructType;
use inkwell::{context::Context, module::Module};
use std::collections::HashMap;
enum ClassKind {
    ReferenceType,
    ValueType,
}
struct ClassIR {
    type_kind: ClassKind,
}
struct Class<'a> {
    ir: ClassIR,
    llvm_type: StructType<'a>,
    is_compleated: bool,
    value_set: HashMap<String, KCRef<Method<'a>>>,
}
impl<'a> Class<'a> {
    fn create(
        ctx: &'a Context,
        module: *const Module,
        path: ClassPath,
        class: ClassIR,
    ) -> Class<'a> {
        //ctx.
        todo!();
    }
}
