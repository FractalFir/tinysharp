use super::r#type::Type;
use super::BlockLink;
use crate::ir::OpBlock;
use crate::ir::OpKind;
use crate::Method;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{
    BasicValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue,
};
use inkwell::{FloatPredicate, IntPredicate};
fn as_u64(i: i64) -> u64 {
    unsafe { std::mem::transmute(i) }
}
enum CMPType {
    GE,
    LE,
}
impl CMPType {
    fn sint_cmp(&self) -> IntPredicate {
        match self {
            Self::GE => IntPredicate::SGE,
            Self::LE => IntPredicate::SLE,
        }
    }
    //fn sint_cmp(&self)
    fn float_cmp(&self) -> FloatPredicate {
        match self {
            Self::GE => FloatPredicate::OGE,
            Self::LE => FloatPredicate::OLE,
        }
    }
}
enum Variable<'a> {
    Int(IntValue<'a>),
    Float(FloatValue<'a>),
    Pointer(PointerValue<'a>),
}
impl<'a> Variable<'a> {
    pub fn as_int(&self) -> Option<IntValue<'a>> {
        if let Self::Int(int) = self {
            Some(*int)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<FloatValue<'a>> {
        if let Self::Float(float) = self {
            Some(*float)
        } else {
            None
        }
    }
    pub fn from_bve(bve: BasicValueEnum<'a>) -> Self {
        match bve {
            BasicValueEnum::IntValue(int) => Self::Int(int),
            BasicValueEnum::FloatValue(f) => Self::Float(f),
            BasicValueEnum::PointerValue(p) => Self::Pointer(p),
            _ => todo!("Can't convert {bve:?} to a Variable IR!"),
        }
    }
    pub fn as_bve(&self) -> BasicValueEnum<'a> {
        match self {
            Self::Int(var) => var.as_basic_value_enum(),
            Self::Float(var) => var.as_basic_value_enum(),
            Self::Pointer(var) => var.as_basic_value_enum(),
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
struct VirtualStack {
    state: Vec<usize>,
}
impl VirtualStack {
    fn new() -> Self {
        Self { state: Vec::new() }
    }
    fn pop(&mut self) -> Option<usize> {
        self.state.pop()
    }
    fn push(&mut self, val: usize) {
        self.state.push(val);
    }
}
impl<'a> MethodCompiler<'a> {
    fn get_local_index(&self, loc_index: usize) -> usize {
        self.method.signature().argc() + loc_index
    }
    pub(crate) fn set_at_end_block(&mut self, block_id: usize) {
        self.builder.position_at_end(self.blocks[block_id]);
    }
    pub(crate) fn build_add(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let var_a = &self.variables[index_a];
        let var_b = &self.variables[index_b];
        match var_a {
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let var_a = *var_a;
                let res = self.builder.build_int_add(var_a, var_b, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let var_a = *var_a;
                let res = self.builder.build_float_add(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Adding pointers unsupported!"),
        }
    }
    pub(crate) fn build_neg(&mut self, index_a: usize) -> Option<usize> {
        let var_a = &self.variables[index_a];
        match var_a {
            Variable::Int(var_a) => {
                let var_a = *var_a;
                let res = self
                    .builder
                    .build_int_sub(var_a.get_type().const_zero(), var_a, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_a = *var_a;
                let res = self
                    .builder
                    .build_float_sub(var_a.get_type().const_zero(), var_a, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Negating a poniter is likely invalid!"),
        }
    }
    pub(crate) fn build_mul(&mut self, index_a: usize, index_b: usize) -> Option<usize> {
        let var_a = &self.variables[index_a];
        let var_b = &self.variables[index_b];
        match var_a {
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let var_a = *var_a;
                let res = self.builder.build_int_mul(var_a, var_b, "");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let var_a = *var_a;
                let res = self.builder.build_float_mul(var_a, var_b, "");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
            Variable::Pointer(_) => todo!("Multiplying 2 pointers unsupported!"),
        }
    }
    fn build_cj(
        &mut self,
        index_a: usize,
        index_b: usize,
        b_then: BasicBlock<'a>,
        b_else: BasicBlock<'a>,
        cmp: &CMPType,
    ) -> Option<()> {
        let var_a = &self.variables[index_a];
        let var_b = &self.variables[index_b];
        match var_a {
            Variable::Int(var_a) => {
                let var_b = var_b.as_int()?;
                let var_a = *var_a;
                let cmp = self
                    .builder
                    .build_int_compare(cmp.sint_cmp(), var_b, var_a, "");
                self.builder.build_conditional_branch(cmp, b_then, b_else);
            }
            Variable::Float(var_a) => {
                let var_b = var_b.as_float()?;
                let var_a = *var_a;
                let cmp = self
                    .builder
                    .build_float_compare(cmp.float_cmp(), var_b, var_a, "");
                self.builder.build_conditional_branch(cmp, b_then, b_else);
            }
            Variable::Pointer(_) => todo!("Branching using pointers unsported!"),
        }
        Some(())
    }
    pub(crate) fn build_ret(&mut self, index_ret: Option<usize>) {
        if let Some(index_ret) = index_ret {
            let val_ret = &self.variables[index_ret];
            self.builder.build_return(Some(&val_ret.as_bve()));
        } else {
            self.builder.build_return(None);
        }
    }
    fn build_set_local(&mut self, index_a: usize, local_index: usize) -> Option<()> {
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
    fn build_load_local(&mut self, local_index: usize) -> Option<usize> {
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
            match op.kind() {
                OpKind::Add => {
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    virt_stack.push(self.build_add(a, b)?);
                }
                OpKind::Mul => {
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    virt_stack.push(self.build_mul(a, b)?);
                }
                OpKind::LDArg(arg_index) => {
                    virt_stack.push(arg_index);
                }
                OpKind::LDCI32(val) => {
                    self.variables.push(Variable::Int(
                        self.ctx.i32_type().const_int(as_u64(i64::from(val)), false),
                    ));
                    virt_stack.push(self.variables.len() - 1);
                }
                OpKind::Ret => {
                    if op.resolved_type()? == Type::Void {
                        self.build_ret(None);
                    } else {
                        let ret = virt_stack.pop()?;
                        self.build_ret(Some(ret));
                    }
                }
                OpKind::Neg => {
                    let a = virt_stack.pop()?;
                    virt_stack.push(self.build_neg(a)?);
                }
                OpKind::BGE(target) => {
                    let target_index = self.method.get_index_of_block_beginig_at(target);
                    let target = self.blocks[target_index];
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    self.build_cj(
                        a,
                        b,
                        target,
                        self.builder.get_insert_block()?.get_next_basic_block()?,
                        &CMPType::GE,
                    );
                }
                OpKind::BLE(target) => {
                    let target_index = self.method.get_index_of_block_beginig_at(target);
                    let target = self.blocks[target_index];
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    self.build_cj(
                        a,
                        b,
                        target,
                        self.builder.get_insert_block()?.get_next_basic_block()?,
                        &CMPType::LE,
                    );
                }
                OpKind::BR(target) => {
                    let target_index = self.method.get_index_of_block_beginig_at(target);
                    let target = self.blocks[target_index];
                    self.builder.build_unconditional_branch(target);
                }
                OpKind::STLoc(index) => {
                    let a = virt_stack.pop()?;
                    self.build_set_local(a, index)?;
                }
                OpKind::LDLoc(index) => {
                    virt_stack.push(self.build_load_local(index)?);
                }
                _ => todo!("Unsuported OpKind:{:?}", op.kind()),
            }
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
        for param in fnc.get_param_iter() {
            variables.push(Variable::from_bve(param));
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
