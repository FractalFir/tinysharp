use super::{op::OpKind, BlockLink, InstructionIndex, MethodIRError, Signature, StackState, VOp};
#[derive(Debug, Clone)]
pub(crate) struct OpBlock {
    block: VOp,
    state_change: Option<StackState>,
    link_out: BlockLink,
}
impl OpBlock {
    pub(crate) fn resolve(
        &mut self,
        state: &mut StackState,
        sig: &Signature,
    ) -> Result<(), MethodIRError> {
        for mut op in &mut self.block {
            op.resolve(state, sig)?;
        }
        Ok(())
    }
    pub(crate) fn link_out(&self) -> BlockLink {
        self.link_out
    }
    pub(crate) fn from_ops(block_end: InstructionIndex, ops: VOp) -> Self {
        let last = &ops[ops.len() - 1].kind();
        let link_out = if let OpKind::Ret = last {
            BlockLink::Return
        } else if let Some(target) = last.branch_target() {
            BlockLink::Branch(block_end + 1, target)
        } else {
            panic!("Internal error: instruction block should have ended with either Ret, or a branch instruction, but ended with {last:?}!");
        };
        OpBlock {
            link_out,
            state_change: None,
            block: ops,
        }
    }
    pub(crate) fn split(mut self, internal_index: usize) -> (Self, Self) {
        assert!(internal_index < self.block.len());
        todo!("Spliting blocks not supported yet!");
    }
}
