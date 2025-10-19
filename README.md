# iec-61499-fb-rs
Implementation of a basic function block according to the IEC 61499 standard.

## The `Voter` function block
A simple basic function block which showcases the encapsulated nature of the standard, as well as how to handle internal state.

### Static interface declaration
```
           +----------------------+
EVENT --x--|Vote             Voted|-x--- EVENT
        |  |                      | |
EVENT -----|Reset            Ready|---x- EVENT
        |  |                      | | |
        |  +-+                  +-+ | |
        |    |      VOTER       |   | |
        |  +-+                  +-+ | |
        |  |                      | | |
 BOOL --x--|A                State|-x-x- BOOL
        |  |                      |
 BOOL --x--|B                     |
        |  |                      |
 BOOL --x--|C                     |
           +----------------------+
```

### Dynamic interface behaivour
TBD

## Goals
- [ ] evaluate different approaches to implement IEC 61499 based function blocks in Rust
- [ ] implement a basic function block with voting functionality
- [ ] (If there is time) research an implement how to convert the implementation to WebAssembly
- [ ] (Optionally) research how to generate valid Rust function blocks from structured text representations of function blocks

## Sources
- Wikipedia: [IEC 61499](https://en.wikipedia.org/wiki/IEC_61499)
- [Modelling Control Systems Using IEC 61499 2nd Edition](https://doi.org/10.1049/PBCE095E) by Alois Zoitl & Robert Lewis
