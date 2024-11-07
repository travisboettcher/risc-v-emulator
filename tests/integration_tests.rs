#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use risc_v_emulator::processor::Processor;
    use risc_v_emulator::assembler;

    #[test]
    fn test_strlen() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/strlen.s"));
        let bits: Vec<u8> = "hello".chars().map(|c| c as u8).collect();
        let a0 = processor.load_into_memory(bits.as_slice());
        processor.set_register_value(10, a0 as u32);
        processor.execute_instructions();

        assert_eq!(5, processor.get_registry_value(10));
    }

    #[test]
    fn test_strcopy() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/strcopy.s"));
        let bits: Vec<u8> = "hello".chars().map(|c| c as u8).collect();
        let a1 = processor.load_into_memory(bits.as_slice());
        let a0 = a1 + 6;
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, a1 as u32);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 5);
        assert_eq!(bits, result);
    }

    #[test]
    fn test_strncpy() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/strncpy.s"));
        let bits: Vec<u8> = "hello".chars().map(|c| c as u8).collect();
        let a1 = processor.load_into_memory(bits.as_slice());
        let a0 = a1 + 6;
        let a2 = 10u32;
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, a1 as u32);
        processor.set_register_value(12, a2);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 10);
        let expected: Vec<u8> = "hello\0\0\0\0\0".chars().map(|c| c as u8).collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_bubsort() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/bubsort.s"));
        let a0 = processor.load_into_memory(&[
            0, 0, 0, 1, 
            0, 0, 0, 4, 
            0, 0, 0, 3, 
            0, 0, 0, 2, 
            0, 0, 0, 5
        ]);
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, 5);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 20);
        assert_eq!(vec![
            0, 0, 0, 1, 
            0, 0, 0, 2, 
            0, 0, 0, 3, 
            0, 0, 0, 4, 
            0, 0, 0, 5
        ], result);
    }

    #[test]
    fn test_strrev() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/strrev.s"));
        let bits: Vec<u8> = "hello\0".chars().map(|c| c as u8).collect();
        let a0 = processor.load_into_memory(bits.as_slice());
        processor.set_register_value(10, a0 as u32);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 5);
        let expected: Vec<u8> = "olleh".chars().map(|c| c as u8).collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_arraysum() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/arraysum.s"));
        let ints = [
            0, 0, 0, 1, 
            0, 0, 0, 2, 
            0, 0, 0, 3, 
            0, 0, 0, 4, 
            0, 0, 0, 5, 
            0, 0, 0, 6, 
            0, 0, 0, 7, 
            0, 0, 0, 8, 
            0, 0, 0, 9, 
            0, 0, 0, 10
        ];
        let a0 = processor.load_into_memory(&ints);
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, ints.len() as u32);
        processor.execute_instructions();

        let result = processor.get_registry_value(10);
        assert_eq!(55, result);
    }

    #[test]
    fn test_binsearch() {
        let mut processor = Processor::new();

        processor.load_instructions(assemble("examples/binsearch.s"));
        let ints = [
            0, 0, 0, 1,
            0, 0, 0, 2,
            0, 0, 0, 3,
            0, 0, 0, 4,
            0, 0, 0, 5,
            0, 0, 0, 6,
            0, 0, 0, 7,
            0, 0, 0, 8,
            0, 0, 0, 9,
            0, 0, 0, 10
        ];
        let a0 = processor.load_into_memory(&ints);
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, 8);
        processor.set_register_value(12, 10);
        processor.execute_instructions();

        let result = processor.get_registry_value(10);
        assert_eq!(7, result);
    }
    
    #[test]
    fn test_sum10() {
        let mut processor = Processor::new();
        
        processor.load_instructions(assemble("examples/sum10.s"));
        processor.execute_instructions();

        let result = processor.get_registry_value(10);
        assert_eq!(20, result);
    }
    
    fn assemble(file_path: &str) -> Vec<u32> {
        let file = File::open(file_path).expect("no such file");
        let buf = BufReader::new(file);

        let instructions: Vec<String> = buf.lines()
            .flatten()
            .collect();

        assembler::assemble(instructions)
    }
}

