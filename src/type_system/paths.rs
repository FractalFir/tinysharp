const IDENT_SPLIT: &str = "*";
use crate::ir::Signature;
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
    pub fn new(
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
    fn sig_string(&self) -> &str {
        &self.identifier[(self.method_end + IDENT_SPLIT.len())..]
    }
    pub(crate) fn ident(&self) -> &str {
        &self.identifier
    }
}
use std::fmt::{Debug,Formatter};
impl Debug for MethodPath{
    fn fmt(&self,f:&mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        write!(f,"MethodPath{{asm:{},namespace:{},class:{},method:{},sig:{}}}", self.assembly_name(), self.namespace(), self.class_name(),self.method_name(),self.sig_string())
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
    for _ in 0..1_000 {
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
    for _ in 0..1_000 {
        let assembly = rnd_name();
        let class = rnd_name();
        let namespace = rnd_name();
        let method = rnd_name();
        let sig = Signature::new(&sig);
        let class_ref = MethodPath::new(&assembly, &namespace, &class, &method, &sig);
        assert_eq!(assembly, class_ref.assembly_name());
        assert_eq!(class, class_ref.class_name());
        assert_eq!(namespace, class_ref.namespace());
        assert_eq!(method, class_ref.method_name());
        println!("{}", class_ref.ident());
    }
}
