pub mod leitner_system;
pub mod super_memory_2;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum TasksWraper {
    LeitnerSystem(Box<[leitner_system::LeitnerSystem]>),
    SyperMemory2(Box<[super_memory_2::SuperMemory]>),
}
