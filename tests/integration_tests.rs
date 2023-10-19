#[cfg(test)]
mod tests {
    use risc_v_emulator::processor::Processor;

    #[test]
    fn test_strlen() {
        let mut processor = Processor::new();

        processor.load_instructions("examples/strlen.s");
        let bits: Vec<u32> = "hello".chars().map(|c| c as u32).collect();
        let a0 = processor.load_into_memory(bits.as_slice());
        processor.set_register_value(10, a0 as u32);
        processor.execute_instructions();

        assert_eq!(5, processor.get_registry_value(10));
    }

    #[test]
    fn test_strcopy() {
        let mut processor = Processor::new();

        processor.load_instructions("examples/strcopy.s");
        let bits: Vec<u32> = "hello".chars().map(|c| c as u32).collect();
        let a1 = processor.load_into_memory(bits.as_slice());
        let a0 = a1 + 6;
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, a1 as u32);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 5);
        assert_eq!(bits, result);
    }

    #[test]
    fn test_bubsort() {
        let mut processor = Processor::new();

        processor.load_instructions("examples/bubsort.s");
        let a0 = processor.load_into_memory(&[1, 4, 3, 2, 5]);
        processor.set_register_value(10, a0 as u32);
        processor.set_register_value(11, 5);
        processor.execute_instructions();

        let result = processor.get_copy_of_memory(a0..a0 + 5);
        assert_eq!(vec![1, 2, 3, 4, 5], result);
    }
}

