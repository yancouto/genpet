//! # genpet
//! 
//! Generate petgraph graphs using the geng program from nauty.

mod generate;
pub use crate::generate::generate_graphs;
pub use crate::generate::geng::GengOption;