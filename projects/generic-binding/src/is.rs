use alloy::primitives::U256;

use crate::{
    bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager,
    interface::{Task, TaskBindingContract},
};

// Implementación del trait para la estructura genérica Task<INPUT>.
// Se asume que el tipo INPUT puede convertirse en U256, que es lo que espera el binding.
impl<INPUT> TaskBindingContract for Task<INPUT>
where
    INPUT: Into<U256> + Clone,
{
    type Input = INPUT;
    type Task = IIncredibleSquaringTaskManager::Task;

    fn create_task(&self) -> Self::Task {
        IIncredibleSquaringTaskManager::Task {
            numberToBeSquared: self.input.clone().into(),
            taskCreatedBlock: self.taskCreatedBlock,
            quorumNumbers: self.quorumNumbers.clone(),
            quorumThresholdPercentage: self.quorumThresholdPercentage,
        }
    }
}
