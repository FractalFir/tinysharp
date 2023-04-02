use super::r#type::Type;
use super::{op::OpKind, BlockLink, InstructionIndex, MethodIRError, Signature, StackState, VOp};

#[derive(Debug, Clone)]
pub(crate) struct OpBlock {
    pub(crate) block: VOp,
    state_change: Option<StackState>,
    link_out: BlockLink,
    block_beg: InstructionIndex,
}
impl OpBlock {
    pub(crate) fn resolve(
        &mut self,
        mut state: StackState,
        sig: &Signature,
        locals: &[Type],
    ) -> Result<(), MethodIRError> {
        for op in &mut self.block {
            op.resolve(&mut state, sig, locals)?;
        }
        self.state_change = Some(state);
        Ok(())
    }
    pub(crate) fn block_beg(&self) -> InstructionIndex {
        self.block_beg
    }
    pub(crate) fn block_end(&self) -> InstructionIndex {
        self.block_beg + self.block.len()
    }
    pub(crate) fn link_out(&self) -> BlockLink {
        self.link_out
    }
    pub(crate) fn state(&self) -> Option<StackState> {
        self.state_change.clone()
    }
    pub(crate) fn is_resolved(&self) -> bool {
        self.state_change.is_some()
    }
    pub(crate) fn from_ops(block_beg: InstructionIndex, ops: VOp) -> Self {
        let block_end = block_beg + ops.len();
        let last = &ops[ops.len() - 1].kind();
        let link_out = if let OpKind::Ret = last {
            BlockLink::Return
        } else if let Some(target) = last.branch_target() {
            BlockLink::Branch(block_end, target)
        } else {
            BlockLink::Pass
        };
        OpBlock {
            link_out,
            state_change: None,
            block: ops,
            block_beg,
        }
    }
}
