use clap::{Parser, Subcommand};
use risc_v_emulator::{assembler::assemble, processor::Processor};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "riscv-emu")]
#[command(about = "RISC-V RV32I Emulator", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a RISC-V assembly file
    Run {
        /// Path to assembly file (.s)
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Show register contents after execution
        #[arg(short, long)]
        registers: bool,

        /// Show memory range after execution (format: start-end, e.g., 512-520)
        #[arg(short, long)]
        memory: Option<String>,

        /// Enable debug output during execution
        #[arg(short, long)]
        debug: bool,
    },

    /// Show information about the emulator
    Info,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            registers,
            memory,
            debug,
        } => {
            run_assembly_file(file, registers, memory, debug)?;
        }
        Commands::Info => {
            show_info();
        }
    }

    Ok(())
}

fn run_assembly_file(
    file: PathBuf,
    show_registers: bool,
    memory_range: Option<String>,
    _debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read assembly file
    let contents = fs::read_to_string(&file)?;

    // Parse assembly
    let assembly: Vec<String> = contents
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();

    println!("Loading {} from {:?}...", assembly.len(), file);

    // Assemble
    let code = assemble(assembly);

    // Create processor and execute
    let mut processor = Processor::new();
    processor.load_instructions(code);

    println!("Executing...");
    processor.execute_instructions();
    println!("Execution complete.");

    // Show registers if requested
    if show_registers {
        println!("\nRegisters:");
        println!("  x0 (zero) = {}", processor.get_registry_value(0));
        println!("  x1 (ra)   = {}", processor.get_registry_value(1));
        println!("  x2 (sp)   = {}", processor.get_registry_value(2));
        for i in 3..32 {
            let val = processor.get_registry_value(i);
            if val != 0 {
                println!("  x{:<2}       = {}", i, val);
            }
        }
    }

    // Show memory if requested
    if let Some(range_str) = memory_range {
        if let Some((start_str, end_str)) = range_str.split_once('-') {
            let start: usize = start_str.parse()?;
            let end: usize = end_str.parse()?;

            let mem = processor.get_copy_of_memory(start..end);
            println!("\nMemory [{}..{}]:", start, end);
            for (i, byte) in mem.iter().enumerate() {
                if i % 16 == 0 {
                    print!("\n  {:04x}:  ", start + i);
                }
                print!("{:02x} ", byte);
            }
            println!();
        }
    }

    Ok(())
}

fn show_info() {
    println!("RISC-V RV32I Emulator");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Features:");
    println!("  - 32 general-purpose registers (x0-x31)");
    println!("  - 1024 bytes of memory");
    println!("  - Full RV32I base instruction set");
    println!("  - Assembly language support");
    println!();
    println!("Supported Instructions:");
    println!("  Arithmetic: add, sub, addi");
    println!("  Logic: and, or, xor, andi, ori, xori");
    println!("  Shift: sll, srl, sra, slli, srli, srai");
    println!("  Compare: slt, sltu, slti, sltiu");
    println!("  Memory: lw, lh, lb, lbu, lhu, sw, sh, sb");
    println!("  Branch: beq, bne, blt, bge, bltu, bgeu");
    println!("  Jump: jal, jalr");
    println!("  Upper: lui, auipc");
}
