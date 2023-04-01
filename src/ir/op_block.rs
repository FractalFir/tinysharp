use super::r#type::Type;
use super::{
    method::Method, op::OpKind, BlockLink, InstructionIndex, MethodIRError, Signature, StackState,
    VOp,
};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
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
        for mut op in &mut self.block {
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
    pub(crate) fn into_llvm_bb(&self, meth: &Method, block_builder: &mut Builder, ctx: &Context) {
        assert!(
            self.state_change
                .clone()
                .expect("Can't convert unresolved block to LLVM IR")
                .is_empty(),
            "Cross-Block Virtual stack is not supported yet"
        );
        //Current approach does not work for inserting things onto virtual stack and needs to be reworked.
        let mut vstack = VirtStack::empty();
        let mut vars: Vec<InstructionValue> = Vec::new();

        use inkwell::values::AnyValue;
        use inkwell::values::BasicValueEnum;
        for op in &self.block {
            match op.kind() {
                OpKind::Ret => {
                    let ret = op
                        .resolved_type()
                        .expect("Unresolved type during code gen.");
                    if let Type::Void = ret {
                        block_builder.build_return(None);
                    } else {
                        let top_any = (&vars[vstack.pop().expect("Noting to return on stack!")])
                            .as_any_value_enum();
                        let top: BasicValueEnum = top_any
                            .try_into()
                            .expect("Nonreturnable value on the stack!");
                        block_builder.build_return(Some(&top));
                    }
                }
                _ => todo!("Codegen does not support {:?}!", op.kind()),
            }
        }
    }
}

use inkwell::values::InstructionValue;
pub(crate) struct VirtStack {
    state: Vec<usize>,
}
impl VirtStack {
    pub(crate) fn empty() -> Self {
        Self { state: Vec::new() }
    }
    fn pop(&mut self) -> Option<usize> {
        self.state.pop()
    }
    fn push(&mut self, val: usize) {
        self.state.push(val)
    }
}
