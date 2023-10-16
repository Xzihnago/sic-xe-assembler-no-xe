fn main() {
    let source = sic_xe_assembler::loader::read_asm_file("input.txt");

    sic_xe_assembler::parse_sic(source);
}
