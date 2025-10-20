use SimNES::cpu::CPU;

#[test]
fn test_cpu() {
    let mut cpu = CPU::new();
    // LDA
    let data = vec![0xa9, 0x05, 0x00];
    cpu.interpret(&data);
    assert_eq!(cpu.reg_a, 0x05);
    // TAX
    let data2 = vec![0xaa, 0x00];
    cpu.interpret(&data2);
    assert_eq!(cpu.reg_x, 0x05);
}
#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    let data = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    cpu.interpret(&data);

    assert_eq!(cpu.reg_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0xff;
    let data = vec![0xe8, 0xe8, 0x00];
    cpu.interpret(&data);

    assert_eq!(cpu.reg_x, 1)
}
