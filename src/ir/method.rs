use super::{
    op::{Op, OpKind},
    r#type::Type,
    BlockLink, InstructionIndex, MethodIRError, OpBlock, Signature, StackState, VBlocks, VOp,
};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::FunctionType;
use inkwell::values::FunctionValue;
#[derive(Debug)]
pub(crate) struct Method {
    signature: Signature,
    pub(crate) blocks: VBlocks,
    pub(crate) locals: Vec<Type>,
}
fn spilt_into_blocks(ops: &[OpKind]) -> VBlocks {
    //nothing to do for now!
    let mut index: InstructionIndex = 0;
    let mut targets: Vec<InstructionIndex> = Vec::new();
    for op in ops {
        if let Some(target) = op.branch_target() {
            targets.push(target - 1);
            targets.push(index);
        }
        index += 1;
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
    if block.len() > 0 {
        blocks.push(OpBlock::from_ops(index - block.len(), block));
    }
    blocks
}
impl Method {
    pub(crate) fn get_arg_count(&self) -> usize {
        self.signature.argc()
    }
    pub(crate) fn get_index_of_block_beginig_at(&self, index: InstructionIndex) -> usize {
        for block_index in 0..self.blocks.len() {
            if self.blocks[block_index].block_beg() == index {
                return block_index;
            }
        }
        panic!("No block begins at instruction with index {index}!");
    }
    pub(crate) fn get_local_type(&self,index:usize)->Type{
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
                self.resolve_node(
                    child_index,
                    self.blocks[index]
                        .state()
                        .expect("State did not resolve, but no error raised!"),
                )
            }
            BlockLink::Branch(default, target) => {
                let def_index = self.get_index_of_block_beginig_at(default);
                assert_ne!(def_index, index, "Default branch target loops");
                self.resolve_node(
                    def_index,
                    self.blocks[index]
                        .state()
                        .expect("State did not resolve, but no error raised!"),
                )?;
                let target_index = self.get_index_of_block_beginig_at(target);
                assert_ne!(target_index, index, "Target branch loops");
                self.resolve_node(
                    target_index,
                    self.blocks[index]
                        .state()
                        .expect("State did not resolve, but no error raised!"),
                )
            }
            _ => todo!("Resolving block links of type {link:?} is not supported"),
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
    pub(crate) fn into_fn_type<'a>(&self, ctx: &'a Context) -> FunctionType<'a> {
        self.signature.into_fn_type(ctx)
    }
    pub(crate) fn signature(&self) -> &Signature {
        &self.signature
    }
}
