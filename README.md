# genpet

Generate petgraph graphs with geng.

## Examples
    
```rust
use genpet::generate_graphs;
use genpet::GengOption;

let graphs = generate_graphs(3, &[]);

for graph in graphs {
    println!("{}", graph.node_count());
}

let graphs = generate_graphs(4, &[GengOption::Connected, GengOption::Chordal]);
for graph in graphs {
    println!("{}", graph.node_count());
}
```

## Testing locally

- Install Rust and Cargo
- Install nauty
- Run `cargo test`