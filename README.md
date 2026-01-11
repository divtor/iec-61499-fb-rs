# iec-61499-fb-rs
Conceptual implementation of basic function block types according to the `IEC 61499` standard.

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

### Encapsulated functionality

#### Vote Algorithm
```
ALGORITHM VoteAlg IN ST:
    State := (A AND B) OR (A AND C) OR (B AND C);
END_ALGORITHM
```

#### Reset Algorithm
```
ALGORITHM ResetAlg IN ST:
    State := FALSE;
END_ALGORITHM
```

## Goals
- [x] basic implementation of the voter function block
- [x] concept of function block handling (see `Fb` trait) 
- [x] concept of `Event` and `Data` connections
- [x] basic `Runtime` to test data and event propergation
- [x] event function blocks (E_SWITCH, E_CTU, E_SR)
- [x] connections using `Id`s instead of references
- [x] test event function blocks

## Non-Goals
- Implementing a fully featured `IEC 61499` solution for basic function block types
- Formally defining execution semantics for function block types
- Composite function block types
- Service interface function block types
    - by extension: supporting hardware communication

## Sources

### IEC 61499
- [Wikipedia](https://en.wikipedia.org/wiki/IEC_61499)
- [Modelling Control Systems Using IEC 61499 2nd Edition](https://doi.org/10.1049/PBCE095E) by Alois Zoitl & Robert Lewis
