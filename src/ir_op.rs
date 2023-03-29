pub(crate) use crate::ir::*;
pub(crate) use crate::ir::{op::{OpKind,Op},r#type::Type,MethodIRError,StackState,Signature,op_block::OpBlock};
// use inkwhell::module::Module;
#[derive(Debug)]
pub(crate) struct Method {
    signature: Signature,
    blocks: VBlocks,
}
fn spilt_into_blocks(ops: &[OpKind]) -> VBlocks {
    //nothing to do for now!
    let mut block = VOp::new();
    let mut blocks = VBlocks::new();
    let mut index:InstructionIndex = 0;
    for op in ops {
        block.push(Op::from_kind(*op));
        if let Some(target) = op.branch_target() {
            blocks.push(OpBlock::from_ops(index - 1,block.clone()));
            block.clear();
        }
        index+=1;
    }
    let mut targets:Vec<InstructionIndex> = Vec::new();
    for block in blocks.iter(){
        match block.link_out(){
            BlockLink::Return=>(),
            BlockLink::Branch(a,b)=>{
                targets.push(a);
                targets.push(b);
            },
        }
    }
    let mut idx = 0;
    while idx < blocks.len(){
        
        idx += 1;
    }
    if block.len() > 0{
        blocks.push(OpBlock::from_ops(index - 1,block));
    }
    blocks
}
impl Method {
    fn resolve(&mut self) -> Result<(), MethodIRError> {
        if self.blocks.len() <= 1 {
            self.blocks[0].resolve(&mut StackState::default(), &self.signature)?;
        } else {
            todo!("Multiple Op Blocks unsupported!");
        }
        Ok(())
    }
    pub(crate) fn from_ops(sig: (&[Type], Type), ops: &[OpKind]) -> Result<Self, MethodIRError> {
        let blocks: VBlocks = spilt_into_blocks(ops);
        let mut res = Self {
            blocks,
            signature: Signature::new(sig),
        };
        println!("Before Resolution:\n{res:?}");
        res.resolve()?;
        println!("After Resolution:\n{res:?}");
        todo!();
        Ok(res)
    }
    /*
    pub fn emmit_llvm(&mut self,module:&mut Module){

    }*/
}
