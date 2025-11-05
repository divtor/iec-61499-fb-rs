# iec-61499-fb-rs
Implementation of `BasicFunctionBlock`s according to the IEC 61499 standard.

## The `Voter` function block
Simple function block which will be used to showcase the standard, as well as how to handle internal state.

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
- [x] basic implementation of the voter function block
- [ ] proposal of `BasicFunctionBlock` implementation
- [ ] proposal of `Connection` implementation
- [ ] (?) convert `BasicFunctionBlock`s to `WebAssembly`
- [ ] (??) generate `BasicFunctionBlock`s from structured text

## Sources

### IEC 61499
- [Wikipedia](https://en.wikipedia.org/wiki/IEC_61499)
- [Modelling Control Systems Using IEC 61499 2nd Edition](https://doi.org/10.1049/PBCE095E) by Alois Zoitl & Robert Lewis
