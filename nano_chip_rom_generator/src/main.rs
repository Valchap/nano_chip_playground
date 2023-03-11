fn main() {
    if let Some(input_file) = std::env::args().nth(1) {
        match std::fs::read(input_file) {
            Ok(raw_binary) => {
                let mut vhdl_str = String::new();

                for u16index in 0..(raw_binary.len() / 2) {
                    let mut instruction =
                        (raw_binary[u16index] as u16) << 8 | raw_binary[u16index + 1] as u16;

                    instruction &= 0b0011_1111_1111_1111; // Mask so that the instruction is on 14 bits, (opcodes are limited to 6 bits)

                    vhdl_str.push_str(&format!(
                        "\"{:014b}\" when \"{:08b}\",\n",
                        instruction, u16index as u8
                    ));
                }

                vhdl_str.push_str("\"10001011111111\" when others;\n");

                println!("{vhdl_str}");
            }
            Err(errmsg) => {
                println!("Error, can't read input file : {errmsg}");
            }
        }
    } else {
        println!("Error, input file is needed as first argument");
    }
}
