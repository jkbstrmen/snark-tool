# snark-tool

Snark tool is simple tool for snarks analysis. Snark is [cubic graph](https://en.wikipedia.org/wiki/Cubic_graph) with [girth](https://en.wikipedia.org/wiki/Girth_(graph_theory)) at least 5 and cyclic edge connectivity at least 4. 

## Use as library

Include snark-tool as a dependency in your Cargo.toml configuration file.

```toml
[dependencies]
...
snark-tool = "0.4.0"
```

Then simply use needed structs or functions in your code.

```rust
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::io::reader_g6::G6Reader;

pub fn read_graph(graph_g6: &String) {
    let graph: SimpleGraph = G6Reader::read_graph(graph_g6).unwrap();
}
```

For more examples see [examples](https://github.com/jkbstrmen/snark-tool/tree/develop/examples) folder.

## Use as binary

To run snark-tool at first we have to define configuration file in YAML format. Here we can specify what we want to do. 

```yaml
version: 0.1

procedures:
  - proc-type: read
    config:
      file: Generated_100_36vert_snarks.g6
      graph-format: g6

  # means regular 3-edge coloring
  - proc-type: colour
    config:
      colouriser-type: dfs

  - proc-type: write
    config:
      file: output.json
      graph-format: s6
      # if set to true, file format will be json for g6/s6 graph format
      with-properties: true # default true

  - proc-type: count
```
Basic usage is that snark-tool takes input file, read graphs from specified format. Then applies/perform specified procedures (in this case edge colouring) over graphs and at the end writes graphs to output file.

When we have specified configuration file with name snark-tool.yml we can start snark-tool using command:

    ./snark-tool run snark-tool.yml


For full description of available procedures and its configuration see [procedures.md](https://github.com/jkbstrmen/snark-tool/blob/develop/procedures.md).

## License
Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## To be noted
Package and all its parts are WIP.