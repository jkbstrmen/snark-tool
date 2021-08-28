pub mod critical_prop;
pub mod critical_prop_parallel;
pub mod edge_subcriticality_solver;
pub mod error;
pub mod resistance;
pub mod resistibility;
pub mod stable_and_critical_prop;
pub mod stable_and_critical_prop_parallel;
pub mod stable_prop;

pub trait CriticalProperties {
    fn is_critical(&mut self) -> bool;
    fn is_cocritical(&mut self) -> bool;
    fn is_vertex_subcritical(&mut self) -> bool;
    fn is_edge_subcritical(&mut self) -> bool;
    fn is_acritical(&mut self) -> bool;
}
