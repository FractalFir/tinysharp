use super::{
    op::{Op, OpKind},
    r#type::Type,
    BlockLink, InstructionIndex, MethodIRError, OpBlock, Signature, StackState, VBlocks, VOp,
};

use inkwell::context::Context;
use inkwell::types::FunctionType;

#[derive(Debug)]
pub(crate) struct Method {
    signature: Signature,
    pub(crate) blocks: VBlocks,
    pub(crate) locals: Vec<Type>,
}
fn spilt_into_blocks(ops: &[OpKind]) -> VBlocks {
    //nothing to do for now!
    let mut targets: Vec<InstructionIndex> = Vec::new();
    for (index, op) in ops.iter().enumerate() {
        if let Some(target) = op.branch_target() {
            targets.push(target - 1);
            targets.push(index);
        }
    }
    let mut block = VOp::new();
    let mut blocks = VBlocks::new();
    let mut index: InstructionIndex = 0;
    for op in ops {
        block.push(Op::from_kind(*op));
        for target in &targets {
            if *target == index {
                blocks.push(OpBlock::from_ops(index + 1 - block.len(), block.clone()));
                block.clear();
                break;
            }
        }
        index += 1;
    }
    if !block.is_empty() {
        blocks.push(OpBlock::from_ops(index - block.len(), block));
    }
    blocks
}
impl Method {
    pub(crate) fn get_index_of_block_beginig_at(&self, index: InstructionIndex) -> usize {
        for block_index in 0..self.blocks.len() {
            if self.blocks[block_index].block_beg() == index {
                return block_index;
            }
        }
        panic!("No block begins at instruction with index {index}!");
    }
    pub(crate) fn get_local_type(&self, index: usize) -> Type {
        self.locals[index]
    }
    fn resolve_node(
        &mut self,
        index: usize,
        parrent_state: StackState,
    ) -> Result<(), MethodIRError> {
        if self.blocks[index].is_resolved() {
            return Ok(());
        }
        self.blocks[index].resolve(parrent_state, &self.signature, &self.locals)?;
        let link = self.blocks[index].link_out();
        match link {
            BlockLink::Return => Ok(()),
            BlockLink::Pass => {
                let child_beg_index = self.blocks[index].block_end();
                let child_index = self.get_index_of_block_beginig_at(child_beg_index);
                let Some(state) = self.blocks[index].state() else {
                   return Err(MethodIRError::StateUnresolvedNoError);
                };
                self.resolve_node(child_index, state)
            }
            BlockLink::Branch(default, target) => {
                let def_index = self.get_index_of_block_beginig_at(default);
                let Some(state) = self.blocks[index].state() else {
                   return Err(MethodIRError::StateUnresolvedNoError);
                };
                self.resolve_node(def_index, state.clone())?;
                let target_index = self.get_index_of_block_beginig_at(target);
                self.resolve_node(target_index, state)
            } // _ => todo!("Resolving block links of type {link:?} is not supported"),
        }
    }
    fn resolve(&mut self) -> Result<(), MethodIRError> {
        self.resolve_node(0, StackState::default())
    }
    pub(crate) fn from_ops(
        sig: (&[Type], Type),
        ops: &[OpKind],
        locals: &[Type],
    ) -> Result<Self, MethodIRError> {
        let blocks: VBlocks = spilt_into_blocks(ops);
        let mut res = Self {
            blocks,
            signature: Signature::new(sig),
            locals: locals.into(),
        };
        res.resolve()?;
        Ok(res)
    }
    pub(crate) fn as_fn_type<'a>(&self, ctx: &'a Context) -> FunctionType<'a> {
        self.signature.as_fn_type(ctx)
    }
    pub(crate) fn signature(&self) -> &Signature {
        &self.signature
    }
}
