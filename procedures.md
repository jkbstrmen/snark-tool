# Basic procedures:

## Procedure _*read*_
Read graph from input file from specified format. This procedure can reuse output file of write procedure written in json format along with graph properties.  

_*required configurations:*_
- **file**
- **graph-format**  
  - options: **g6, s6, ba, json**
  - default: g6

_*optional configurations:*_
- **number-of-graphs**  
  - default is all graphs from input file

#### Example
```yaml
procedures:
  ...
  - proc-type: read
    config:
      file: graphs.g6
      graph-format: g6 # options: g6, s6, ba, json; default: g6
      # optional
      number-of-graphs: 10 # if not set, take all
  ...
```

## Procedure _*write*_
Write graphs to output file in specified format. When set **with-properties** parameter to **true** output file will be in JSON format containing graph in specified format and along with it all its properties computed by snark-tool or given in input file. Output file of this procedure in JSON format can be reused by read procedure as input file. 

_*optional configurations:*_
- **file**   
  - default is 'output'
- **graph-format**   
  - options: **g6, s6, ba** 
  - default is same as input
- **with-properties**
  - options: **true/false**
  - option true available only for G6 and S6 graph formats

#### Example
```yaml
procedures:
  ...
  - proc-type: write
    config:
      file: output.json
      graph-format: s6
      # if set to true, file format will be json for
      # g6/s6 graph format
      with-properties: true # default true
  ...
```

## Procedure _*colour*_ 

Description

_*required configurations:*_
- **colouriser-type**  
  - options: **dfs, sat, cvd, cvd-dfs, cvd-sat, matching**
  - defalt: dfs

#### Example
```yaml
procedures:
  ...
  - proc-type: colour
    config:
      # options: dfs, sat, cvd, cvd-dfs, cvd-sat,
      # matching; default: dfs
      colouriser-type: dfs
  ...
```

## Procedure _*filter*_ 

Description

Configuration

#### Example
```yaml
procedures:
  ...
  - proc-type: filter
    config:
      filter-by:
        costable: true
        oddness: 2
        edge-resistibility-index:
          comparator: ">" # options: >, <, =, !=, >=, <=
          value: 0
  ...
```

## Procedure _*chromatic-properties*_ 

Description

Configuration

#### Example
```yaml
procedures:
  ...
  - proc-type: chromatic-properties
    config:
      # options: graph-based, batch-based, none
      # default: none
      parallelization: graph-based
      # default - number of cpus available
      max-threads: 8
      # options: dfs, sat; default: dfs
      colouriser-type: dfs
      properties:
        - critical
        - cocritical
        - vertex-subcritical
        - edge-subcritical
        - acritical
        - stable
        - costable
        - resistance
        - oddness
        - girth
        - cyclic-edge-connectivity
        - edge-resistibility # for all edges
        - vertex-resistibility # for all vertices
  ...
```