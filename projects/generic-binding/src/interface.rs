use alloy::primitives::Bytes;

// Estructura genérica en el SDK para representar una tarea.
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Task<INPUT> {
    pub input: INPUT,
    pub taskCreatedBlock: u32,
    pub quorumNumbers: Bytes,
    pub quorumThresholdPercentage: u32,
}

// Trait para definir el contrato de binding de la tarea.
// Este trait permite transformar la representación genérica en la
// estructura específica requerida (que vendría del binding del contrato).
pub trait TaskBindingContract {
    type Input;
    type Task;

    /// Convierte la tarea (con input genérico) en la representación de binding
    /// esperada por el contrato.
    fn create_task(&self) -> Self::Task;
}
