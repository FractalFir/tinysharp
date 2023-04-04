const IDENT_SPLIT:&str = "*";
use crate::ir::method::Method;
use crate::ir::Signature;
struct AssemblyPrototype{
    types:ClassPrototype,
}
struct ClassPrototype{
    method:Method,
}
pub(crate) struct ClassRef{
    identifier:String,
    namespace_end:usize,
}
impl ClassRef{
    pub fn new(namespace:&str,class_name:&str)->Self{
        let namespace_end = namespace.len();
        let mut identifier = String::with_capacity(namespace_end + IDENT_SPLIT.len() + class_name.len());
        identifier += namespace; 
        identifier += IDENT_SPLIT;
        identifier += class_name;
        ClassRef{identifier,namespace_end}
    }
    pub fn namespace(&self)->&str{
        &self.identifier[0..self.namespace_end]
    }
    pub fn class_name(&self)->&str{
        &self.identifier[(self.namespace_end + IDENT_SPLIT.len())..]
    }
    pub(crate) fn ident(&self)->&str{&self.identifier}
}
pub(crate) struct MethodRef{
    identifier:String,
    namespace_end:usize,
    class_end:usize,
    method_end:usize,
    sig:Signature
}
impl MethodRef{
    pub fn new(namespace:&str,class_name:&str,method_name:&str,sig:&Signature)->Self{
        let namespace_end = namespace.len();
        let class_end = namespace_end + IDENT_SPLIT.len() + class_name.len();
        let method_end = class_end + IDENT_SPLIT.len() + method_name.len();
        let sig_mangle = sig.to_mangle_string();
        let mut identifier = String::with_capacity(method_end + IDENT_SPLIT.len() + sig_mangle.len());
        identifier += namespace; 
        identifier += IDENT_SPLIT;
        identifier += class_name;
        identifier += IDENT_SPLIT;
        identifier += method_name;
        identifier += IDENT_SPLIT;
        identifier += &sig_mangle;
        MethodRef{identifier,namespace_end,class_end,method_end,sig:sig.clone()}
    }
    pub fn namespace(&self)->&str{
        &self.identifier[0..self.namespace_end]
    }
    pub fn class_name(&self)->&str{
        &self.identifier[(self.namespace_end + IDENT_SPLIT.len())..self.class_end]
    }
    pub fn method_name(&self)->&str{
        &self.identifier[(self.class_end + IDENT_SPLIT.len())..self.method_end]
    }
    pub(crate) fn ident(&self)->&str{&self.identifier}
}
#[cfg(test)]
fn rnd_name()->String{
    const NAMES:[&str;10] = ["Kiwi","Apple","Pear", "Banana","Pineapple","Blueberry", "Strawberry","Peach","Orange", "Targentine"];
    let index = crate::ir::op_test::rnd_u32() as usize % NAMES.len();
    let index2 = crate::ir::op_test::rnd_u32() as usize % NAMES.len();
    NAMES[index].to_owned() + NAMES[index2]
}
#[cfg(test)]#[test]
fn class_ref(){
    for i in 0..1_000{
        let class = rnd_name();
        let namespace = rnd_name();
        let class_ref = ClassRef::new(&namespace,&class);
        assert_eq!(class,class_ref.class_name());
        assert_eq!(namespace,class_ref.namespace());
    }
}
#[cfg(test)]#[test]
fn method_ref(){
    use crate::ir::r#type::Type;
    let args: [Type; 8] = [Type::I64, Type::U64,Type::F64,Type::I32,Type::U32,Type::U16,Type::U8,Type::Bool];
    let sig: (&[Type], Type) = (&args, Type::I8);
    for i in 0..1_000{
        let class = rnd_name();
        let namespace = rnd_name();
        let method = rnd_name();
        let sig = Signature::new(sig);
        let class_ref = MethodRef::new(&namespace,&class,&method,&sig);
        assert_eq!(class,class_ref.class_name());
        assert_eq!(namespace,class_ref.namespace());
        assert_eq!(method,class_ref.method_name());
    }
}
