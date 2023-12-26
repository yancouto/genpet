//! # genpet
//! 
//! Generate petgraph graphs using the geng program from nauty.
//!
//! ## Example
//! ```
//! use genpet::generate_graphs;
//! use genpet::GengOption;
//! 
//! let graphs = generate_graphs(3, &[]);
//! 
//! for graph in graphs {
//!     println!("{}", graph.node_count());
//! }
//! 
//! let graphs = generate_graphs(4, &[GengOption::Connected, GengOption::Chordal]);
//! for graph in graphs {
//!     println!("{}", graph.node_count());
//! }
//! ```
mod generate;
pub use crate::generate::generate_graphs;
pub use crate::generate::geng::GengOption;