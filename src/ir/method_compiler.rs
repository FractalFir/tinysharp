use super::compile_variable::Variable;
use inkwell::types::IntType;
use inkwell::values::IntValue;
use super::r#type::Type;
use super::BlockLink;
use super::OpBlock;
use super::OpKind;
use crate::Method;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::FunctionValue;
use inkwell::{FloatPredicate, IntPredicate};
fn as_u64(i: i64) -> u64 {
    unsafe { std::mem::transmute(i) }
}
pub(crate) enum CMPType {
    GE,
    LE,
    EQ,
    NE,
    LT,
    GT,
}
impl CMPType {
    fn sint_cmp(&self) -> IntPredicate {
        match self {
            Self::GE => IntPredicate::SGE,
            Self::LE => IntPredicate::SLE,
            Self::EQ => IntPredicate::EQ,
            Self::NE => IntPredicate::NE,
            Self::LT => IntPredicate::SLT,
            Self::GT => IntPredicate::SGT,
        }
    }
    fn uint_cmp(&self) -> IntPredicate {
        match self {
            Self::GE => IntPredicate::UGE,
            Self::LE => IntPredicate::ULE,
            Self::EQ => IntPredicate::EQ,
            Self::NE => IntPredicate::NE,
            Self::LT => IntPredicate::ULT,
            Self::GT => IntPredicate::UGT,
        }
    }
    fn float_cmp(&self) -> FloatPredicate {
        match self {
            Self::GE => FloatPredicate::OGE,
            Self::LE => FloatPredicate::OLE,
            Self::EQ => FloatPredicate::OEQ,
            Self::NE => FloatPredicate::ONE,
            Self::LT => FloatPredicate::OLT,
            Self::GT => FloatPredicate::OGT,
        }
    }
}
pub(crate) struct MethodCompiler<'a> {
    ctx: &'a Context,
    method: &'a Method,
    variables: Vec<Variable<'a>>,
    blocks: Vec<BasicBlock<'a>>,
    builder: Builder<'a>,
    // fnc: FunctionValue<'a>,
}
pub(crate) struct VirtualStack {
    state: Vec<usize>,
}
impl VirtualStack {
    fn new() -> Self {
        Self { state: Vec::new() }
    }
    pub(crate) fn pop(&mut self) -> Option<usize> {
        self.state.pop()
    }
    pub(crate) fn push(&mut self, val: usize) {
        self.state.push(val);
    }
}
impl<'a> MethodCompiler<'a> {
    pub(crate) fn add_const_i32(&mut self, val: i32) -> usize {
        let var = self.ctx.i32_type().const_int(as_u64(i64::from(val)), false);
        self.variables.push(Variable::Int(var));
        self.variables.len() - 1
    }
    pub(crate) fn get_next_block(&self) -> Option<BasicBlock<'a>> {
        self.builder.get_insert_block()?.get_next_basic_block()
    }
    pub(crate) fn block_at(&self, block_index: usize) -> Option<BasicBlock<'a>> {
        self.blocks.get(block_index).copied()
    }
    pub(crate) fn method(&self) -> &Method {
        self.method
    }
    fn get_local_index(&self, loc_index: usize) -> usize {
        self.method.signature().argc() + loc_index
    }
    pub(crate) fn set_at_end_block(&mut self, block_id: usize) {
        self.builder.position_at_end(self.blocks[block_id]);
    }
    pub(crate) fn unconditional_branch(&mut self, target: BasicBlock) {
        self.builder.build_unconditional_branch(target);
    }
    pub(crate) fn add(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let int_b = var_b.as_any_int()?;
                let res = self.builder.build_int_add(int_a, int_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let res = self.builder.build_float_add(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn or(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let int_b = var_b.as_any_int()?;
                let res = self.builder.build_or(int_a, int_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(_) => panic!("Can't or 2 floats together!"),
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn xor(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let int_b = var_b.as_any_int()?;
                let res = self.builder.build_xor(int_a, int_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(_) => panic!("Can't or 2 floats together!"),
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn shl(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let int_b = var_b.as_any_int()?;
                let res = self.builder.build_left_shift(int_a, int_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(_) => panic!("Can't or 2 floats together!"),
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn not(&mut self, index_a: usize) -> Option<usize> {
        let var_a = self.variables[index_a];
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let res = self.builder.build_not(int_a, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(_) => panic!("Can't not a float!"),
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn and(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(int_a) | Variable::Int(int_a) => {
                let int_b = var_b.as_any_int()?;
                let res = self.builder.build_and(int_a, int_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(_) => panic!("Can't and 2 floats together!"),
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn sub(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::Int(int_a) | Variable::UInt(int_a) => {
                let var_b = var_b.as_any_int()?;
                let res = self.builder.build_int_sub(int_a, var_b, "");
                self.variables.push(var_a.matching_int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let res = self.builder.build_float_sub(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Subtracting pointers unsupported!"),
        }
    }
    pub(crate) fn neg(&mut self, index_a: usize) -> Option<usize> {
        let var_a = self.variables[index_a];
        match var_a {
            Variable::Int(var_a) => {
                let res = self
                    .builder
                    .build_int_sub(var_a.get_type().const_zero(), var_a, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::UInt(_) => panic!("Attempting to negate unsigned integer!"),
            Variable::Float(var_a) => {
                let res = self
                    .builder
                    .build_float_sub(var_a.get_type().const_zero(), var_a, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Negating a poniter is likely invalid!"),
        }
    }
    pub(crate) fn mul(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(var_a) => {
                let var_b = var_b.as_uint()?;
                let res = self.builder.build_int_mul(var_a, var_b, "");
                self.variables.push(Variable::UInt(res));
                Some(self.variables.len() - 1)
            }
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let res = self.builder.build_int_mul(var_a, var_b, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let res = self.builder.build_float_mul(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Multiplying 2 pointers unsupported!"),
        }
    }
    pub(crate) fn div(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(var_a) => {
                let var_b = var_b.as_uint()?;
                let res = self.builder.build_int_unsigned_div(var_a, var_b, "");
                self.variables.push(Variable::UInt(res));
                Some(self.variables.len() - 1)
            }
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let res = self.builder.build_int_signed_div(var_a, var_b, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let res = self.builder.build_float_div(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Dividing 2 pointers unsupported!"),
        }
    }
    pub(crate) fn rem(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::UInt(var_a) => {
                let var_b = var_b.as_uint()?;
                let res = self.builder.build_int_unsigned_rem(var_a, var_b, "");
                self.variables.push(Variable::UInt(res));
                Some(self.variables.len() - 1)
            }
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let res = self.builder.build_int_signed_rem(var_a, var_b, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let res = self.builder.build_float_rem(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Geting a reminder of 2 pointers unsupported!"),
        }
    }
    pub(crate) fn cj(
        &mut self,
        index_a: usize,
        index_b: usize,
        b_then: BasicBlock<'a>,
        b_else: BasicBlock<'a>,
        cmp: &CMPType,
    ) -> Option<()> {
        let (var_a, var_b) = (self.variables[index_a], self.variables[index_b]);
        match var_a {
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let cmp = self
                    .builder
                    .build_int_compare(cmp.sint_cmp(), var_a, var_b, "");
                self.builder.build_conditional_branch(cmp, b_then, b_else);
            }
            Variable::UInt(var_a) => {
                let var_b = var_b.as_uint()?;
                let cmp = self
                    .builder
                    .build_int_compare(cmp.uint_cmp(), var_a, var_b, "");
                self.builder.build_conditional_branch(cmp, b_then, b_else);
            }
            Variable::Float(float_a) => {
                let var_b = var_b.as_float()?;
                let cmp = self
                    .builder
                    .build_float_compare(cmp.float_cmp(), float_a, var_b, "");
                self.builder.build_conditional_branch(cmp, b_then, b_else);
            }
            Variable::Pointer(_) => todo!("Branching using pointers unsported!"),
        }
        Some(())
    }
    pub(crate) fn ret(&mut self, index_ret: Option<usize>) {
        if let Some(index_ret) = index_ret {
            let val_ret = &self.variables[index_ret];
            self.builder.build_return(Some(&val_ret.as_bve()));
        } else {
            self.builder.build_return(None);
        }
    }
    fn convert_to_int(&mut self, src_index:usize,target:IntType<'a>)->usize{
        let src = self.variables[src_index];
        match src{
            Variable::Int(src_int)=>{
                if src_int.get_type().get_bit_width() < target.get_bit_width(){
                    self.variables.push(Variable::Int(self.builder.build_int_s_extend(src_int,target,"")));;
                }
                else{
                    self.variables.push(Variable::Int(self.builder.build_int_truncate(src_int,target,"")));
                }
                self.variables.len() - 1 
            },
            Variable::UInt(src_int)=>{
                if src_int.get_type().get_bit_width() < target.get_bit_width(){
                    self.variables.push(Variable::Int(self.builder.build_int_z_extend(src_int,target,"")));
                }
                else{
                    self.variables.push(Variable::Int(self.builder.build_int_truncate(src_int,target,"")));
                }
                self.variables.len() - 1 
            }
            _=>panic!("Converting from anything else than signed integers is not supported!"),
        }
    }
    pub(crate) fn convert(&mut self, src_index:usize,target:Type)->Option<usize>{
        if(target.is_int()){
            Some(self.convert_to_int(src_index,target.as_int(self.ctx).unwrap()))
        }
        else{
            panic!("Can't convert type:{target:?}");
        }
    }
    pub(crate) fn set_local(&mut self, index_a: usize, local_index: usize) -> Option<()> {
        let var_a = &self.variables[index_a];
        let Variable::Pointer(ptr) = self.variables[self.get_local_index(local_index)] else {
            return None;
        };
        match var_a {
            Variable::Int(int) => {
                self.builder.build_store(ptr, *int);
                Some(())
            }
            _ => todo!("Can't store local!"),
        }
    }
    pub(crate) fn load_local(&mut self, local_index: usize) -> Option<usize> {
        let Variable::Pointer(ptr) = self.variables[self.get_local_index(local_index)] else {
            return None;
        };
        let t = self.method.get_local_type(local_index);
        let t = t
            .as_llvm_basic_type(self.ctx)
            .expect("Invalid local var type!");
        let res = self.builder.build_load(t, ptr, "");
        self.variables.push(Variable::from_bve(res));
        Some(self.variables.len() - 1)
    }
    pub(crate) fn block_ops(&mut self, src_block: &OpBlock, index: usize) -> Option<()> {
        let mut virt_stack = VirtualStack::new();
        self.set_at_end_block(index);
        for op in &src_block.block {
            use crate::ir::op_compiler::compile_op;
            compile_op(self, op, &mut virt_stack);
        }
        if let BlockLink::Pass = src_block.link_out() {
            self.builder.build_unconditional_branch(
                self.builder.get_insert_block()?.get_next_basic_block()?,
            );
        }
        Some(())
    }
    pub(crate) fn new(ctx: &'a Context, fnc: FunctionValue<'a>, method: &'a Method) -> Self {
        let builder = ctx.create_builder();
        let mut blocks = Vec::new();
        let mut variables = Vec::new();
        let mut params = fnc.get_param_iter();
        for t in method.signature().args() {
            let param = params.next().expect("Argument count mismatch!");
            variables.push(Variable::from_bve_typed(param,t));
        }
        let init_block = ctx.append_basic_block(fnc, "locals_init");
        builder.position_at_end(init_block);
        for local in &method.locals {
            let local = local
                .as_llvm_basic_type(ctx)
                .expect("Invalid local var type!");
            let ptr = builder.build_alloca(local, "");
            variables.push(Variable::Pointer(ptr));
        }
        for _ in 0..method.blocks.len() {
            blocks.push(ctx.append_basic_block(fnc, ""));
        }
        builder.build_unconditional_branch(blocks[0]);
        let mut res = Self {
            ctx,
            method,
            variables,
            blocks,
            builder,
        };
        for (index, block) in method.blocks.iter().enumerate() {
            res.block_ops(block, index);
        }
        res
    }
}
enum MethodCompileError{
    NoItemOnStack,
}
