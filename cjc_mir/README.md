Some of the key characteristics of MIR are:

- It is based on a control-flow graph.
- It does not have nested expressions.
- All types in MIR are fully explicit.

Key MIR vocabulary

 - Basic blocks: units of the control-flow graph, consisting of:
    - statements: actions with one successor
    - terminators: actions with potentially multiple successors; always at the end of a block

## Python Samples

```rust
LoadName { name: "print", scope: Free },
LoadConst { value: String { value: "hello,world" } },
CallFunction { typ: Positional(1) },

Pop,
LoadConst { value: None },
ReturnValue
```