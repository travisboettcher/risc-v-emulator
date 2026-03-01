# RISC-V Emulator

A RISC-V RV32I instruction set emulator written in Rust.

## Features

- ✅ Full RV32I base instruction set support (30+ instructions)
- ✅ 32 general-purpose registers with x0 hardwired to zero
- ✅ 1KB addressable memory
- ✅ Built-in assembler with label support
- ✅ Pseudo-instruction expansion (`li`, `mv`, `ret`, `j`, etc.)
- ✅ Comprehensive test suite (74 unit + 8 integration tests)
- ✅ Pure Rust implementation

## Quick Start

```rust
use risc_v_emulator::{processor::Processor, assembler::assemble};

// Create a new processor
let mut processor = Processor::new();

// Write assembly code
let assembly = vec![
    "addi x5, x0, 10".to_string(),   // x5 = 10
    "addi x6, x0, 20".to_string(),   // x6 = 20
    "add x7, x5, x6".to_string(),    // x7 = x5 + x6 = 30
];

// Assemble and execute
let code = assemble(assembly);
processor.load_instructions(code);
processor.execute_instructions();

// Check results
let result = processor.get_registry_value(7);
println!("x7 = {}", result);  // Outputs: x7 = 30
```

## Supported Instructions

**Arithmetic (R-type):** `add`, `sub`, `and`, `or`, `xor`, `sll`, `srl`, `sra`, `slt`, `sltu`

**Immediate (I-type):** `addi`, `andi`, `ori`, `xori`, `slli`, `srli`, `srai`, `slti`, `sltiu`

**Load/Store:** `lw`, `lh`, `lb`, `lbu`, `lhu`, `sw`, `sh`, `sb`

**Branch:** `beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`

**Jump:** `jal`, `jalr`

**Upper Immediate:** `lui`, `auipc`

**Pseudo-instructions:** `nop`, `li`, `mv`, `not`, `neg`, `j`, `ret`, `call`, and more

## Examples

See the `examples/` directory for working RISC-V assembly programs:

- `strlen.s` - Calculate string length
- `strcopy.s` - Copy a string
- `strrev.s` - Reverse a string
- `bubsort.s` - Bubble sort algorithm
- `arraysum.s` - Sum array elements
- `binsearch.s` - Binary search

Run tests to see them in action:

```bash
cargo test --test integration_tests
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --no-deps --open
```

## References

- [RISC-V Specification](https://riscv.org/technical/specifications/)
- [RV32I Instruction Set](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf)

## License

Licensed under MIT or Apache-2.0 at your option.
