use criterion::{black_box, criterion_group, criterion_main, Criterion};
use risc_v_emulator::{processor::Processor, assembler::assemble};

fn bench_instruction_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("instruction_execution");

    // Benchmark arithmetic instruction
    group.bench_function("addi", |b| {
        b.iter(|| {
            let mut proc = Processor::new();
            let code = assemble(vec!["addi x5, x0, 10".to_string()]);
            proc.load_instructions(code);
            black_box(proc.get_registry_value(5));
        });
    });

    group.finish();
}

fn bench_assembler(c: &mut Criterion) {
    let mut group = c.benchmark_group("assembler");

    group.bench_function("assemble_10_instructions", |b| {
        let asm = vec![
            "addi x5, x0, 10".to_string(),
            "addi x6, x0, 20".to_string(),
            "add x7, x5, x6".to_string(),
            "sub x8, x7, x5".to_string(),
            "and x9, x7, x8".to_string(),
            "or x10, x9, x5".to_string(),
            "xor x11, x10, x6".to_string(),
            "sll x12, x11, x5".to_string(),
            "srl x13, x12, x6".to_string(),
            "jalr x0, x1, 0".to_string(),
        ];

        b.iter(|| {
            black_box(assemble(asm.clone()));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_instruction_execution, bench_assembler);
criterion_main!(benches);
