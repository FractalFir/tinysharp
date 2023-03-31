use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{IntValue,FloatValue,FunctionValue,BasicValueEnum,BasicValue};
use crate::ir::OpBlock;
use crate::ir::OpKind;
use crate::Method;
use super::r#type::Type;
enum Variable<'a>{
    Int(IntValue<'a>),
    Float(FloatValue<'a>),
} 
impl<'a> Variable<'a>{
    pub fn as_int(&self)->Option<IntValue<'a>>{
        if let Self::Int(int) = self{Some(*int)}
        else{None}
    }
    pub fn as_float(&self)->Option<FloatValue<'a>>{
        if let Self::Float(float) = self{Some(*float)}
        else{None}
    }
    pub fn from_bve(bve:BasicValueEnum<'a>)->Self{
        match bve{
            BasicValueEnum::IntValue(int)=>Self::Int(int),
            BasicValueEnum::FloatValue(f)=>Self::Float(f),
            _=>todo!("Can't convert {bve:?} to a Variable IR!"),
        }
    }
    pub fn into_bve(&self)->BasicValueEnum<'a>{
        match self{
            Self::Int(var)=>var.as_basic_value_enum(),
            Self::Float(var)=>var.as_basic_value_enum(),
        }
    }
}
pub(crate) struct MethodCompiler<'a>{
    ctx:&'a Context,
    variables:Vec<Variable<'a>>,
    blocks:Vec<BasicBlock<'a>>,
    builder:Builder<'a>,
    fnc:FunctionValue<'a>
}
struct VirtualStack{
    state:Vec<usize>,
}
impl VirtualStack{
    fn new()->Self{
        Self{state:Vec::new()}
    }
    fn pop(&mut self)->Option<usize>{
        self.state.pop()
    }
    fn push(&mut self,val:usize){
        self.state.push(val)
    }
}
impl<'a> MethodCompiler<'a>{
    pub(crate) fn set_at_end_block(&mut self,block_id:usize){
        self.builder.position_at_end(self.blocks[block_id]);
    }
    pub(crate) fn build_add(&mut self,index_a:usize,index_b:usize)->Option<usize>{
        let var_a = &self.variables[index_a];
        let var_b = &self.variables[index_b];
        match var_a{
            Variable::Int(var_a)=>{
                let var_b = var_b.as_int()?;
                let var_a = *var_a;
                let res = self.builder.build_int_add(var_a,var_b,"");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            },
            Variable::Float(var_a)=>{
                let var_b = var_b.as_float()?;
                let var_a = *var_a;
                let res = self.builder.build_float_add(var_a,var_b,"");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
        }
    }
    pub(crate) fn build_mul(&mut self,index_a:usize,index_b:usize)->Option<usize>{
        let var_a = &self.variables[index_a];
        let var_b = &self.variables[index_b];
        match var_a{
            Variable::Int(var_a)=>{
                let var_b = var_b.as_int()?;
                let var_a = *var_a;
                let res = self.builder.build_int_mul(var_a,var_b,"");
                self.variables.push(Variable::Int(res));
                Some(self.variables.len() - 1)
            },
            Variable::Float(var_a)=>{
                let var_b = var_b.as_float()?;
                let var_a = *var_a;
                let res = self.builder.build_float_mul(var_a,var_b,"");
                self.variables.push(Variable::Float(res));
                Some(self.variables.len() - 1)
            }
        }
    }
    pub(crate) fn build_ret(&mut self,index_ret:Option<usize>){
        println!("Compiling ret!");
        if let Some(index_ret) = index_ret{
            let val_ret = &self.variables[index_ret];
            self.builder.build_return(Some(&val_ret.into_bve()));
        }
        else{
             self.builder.build_return(None);
        }
    }
    pub(crate) fn block_ops(&mut self,src_block:&OpBlock,index:usize)->Option<()>{
        println!("Compiling block of len {}!",src_block.block.len());
        let mut virt_stack = VirtualStack::new();
        self.set_at_end_block(index);
        for op in &src_block.block{
            match op.kind(){
                OpKind::Add=>{
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    virt_stack.push(self.build_add(a,b)?);
                },
                OpKind::Mul=>{
                    let a = virt_stack.pop()?;
                    let b = virt_stack.pop()?;
                    virt_stack.push(self.build_mul(a,b)?);
                },
                OpKind::LDArg(arg_index)=>{
                    virt_stack.push(arg_index);
                },
                OpKind::LDCI32(val)=>{
                    self.variables.push(Variable::Int(self.ctx.i32_type().const_int(val as u64,false)));
                    virt_stack.push(self.variables.len() - 1);
                },
                OpKind::Ret=>{
                    match op.resolved_type()?{
                        Type::Void=>{
                            self.build_ret(None);
                        },
                        _=>{
                            let ret = virt_stack.pop()?;
                            self.build_ret(Some(ret));
                        },
                    }
                    
                },
                _=>todo!("Unsuported OpKind:{:?}",op.kind()),
            }
        }
        Some(())
    }
    pub(crate) fn new(ctx:&'a Context,fnc:&FunctionValue<'a>,met:&Method)->Self{
        let mut builder = ctx.create_builder();
        let mut blocks = Vec::new();
        let mut variables = Vec::new();
        for param in fnc.get_param_iter(){
            variables.push(Variable::from_bve(param));
        }
        for block in 0..met.blocks.len(){
            blocks.push(ctx.append_basic_block(*fnc, "bb"));
        }
        let mut res = Self{ctx,fnc:*fnc,builder,blocks,variables};
        let mut index = 0;
        for block in &met.blocks{
            res.block_ops(&block,index);
            index += 1;
        }
        res
    }
}
