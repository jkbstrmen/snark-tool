## snark-tool

Snark tool is simple tool for snarks analysis. 

To run snark-tool first we have to have configuration file in YAML format. Here we can specify what we 
want to do. 

```yaml
    procedures:
      - proc-type: read
        config:
          file: resources/graphs/petersen.g6
          graph-format: g6
          number-of-graphs: 1
    
      - proc-type: colour
        config:
          colouriser-type: dfs
    
      - proc-type: write
        config:
          file: output.s6
          graph-format: s6
```
 
Basic usage is that snark-tool takes input file, read graphs from specified format. Then applies/perform 
specified procedures over graphs and at the end writes graphs to output file.

### Basic procedures:

#### read
_*required configurations:*_ 
- file
- graph-format  
(available: g6, s6, ba)

_*optional configurations:*_
- number-of-graphs  
(default is all graphs from input file)

#### colour
_*required configurations:*_ 
- colouriser-type  
(available: dfs)

#### write
_*optional configurations:*_
- file   
(default is 'output')
- graph-format   
(Available: g6, s6, ba. Default is same as input)