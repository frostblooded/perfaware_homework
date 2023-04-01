#[derive(Debug, Clone, Copy)]
enum Register {
    AL,
    AX,
    CL,
    CX,
    DL,
    DX,
    BL,
    BX,
    AH,
    SP,
    CH,
    BP,
    DH,
    SI,
    BH,
    DI,
}

const NONWIDE_REGISTER_DECODE_TABLE: [Register; 8] = [
    Register::AL,
    Register::CL,
    Register::DL,
    Register::BL,
    Register::AH,
    Register::CH,
    Register::DH,
    Register::BH,
];

const WIDE_REGISTER_DECODE_TABLE: [Register; 8] = [
    Register::AX,
    Register::CX,
    Register::DX,
    Register::BX,
    Register::SP,
    Register::BP,
    Register::SI,
    Register::DI,
];

impl Register {
    fn decode_reg(byte: &u8, w: &u8) -> Self {
        let reg_value: u8 = (byte >> 3) & 0b111;
        Self::decode_value(&reg_value, w)
    }

    fn decode_rm(byte: &u8, w: &u8) -> Self {
        let reg_value: u8 = byte & 0b111;
        Self::decode_value(&reg_value, w)
    }

    fn decode_value(value: &u8, w: &u8) -> Register {
        assert!((value >> 3) == 0);

        match w {
            0 => NONWIDE_REGISTER_DECODE_TABLE[*value as usize],
            1 => WIDE_REGISTER_DECODE_TABLE[*value as usize],
            _ => panic!("Invalid w value"),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let enum_name: String = format!("{:?}", self);
        write!(f, "{}", enum_name.to_lowercase())
    }
}

fn get_bit(byte: &u8, position: u8) -> u8 {
    (byte >> position) & 1
}

fn is_basic_mov(byte: &u8) -> bool {
    ((byte >> 3) ^ 0b10001) == 0
}

fn is_register_mode(byte: &u8) -> bool {
    (byte >> 6) ^ 0b11 == 0
}

fn main() {
    let mut file_content: Vec<u8> = std::fs::read("inputs/listing_0037_single_register_mov")
        .expect("Could not read input file");

    let second_byte: u8 = file_content
        .pop()
        .expect("File doesn't contain second byte");
    let first_byte: u8 = file_content.pop().expect("File doesn't contain first byte");

    assert!(is_basic_mov(&first_byte));
    assert!(is_register_mode(&second_byte));

    let d: u8 = get_bit(&first_byte, 1);
    let w: u8 = get_bit(&first_byte, 0);

    let reg: Register = Register::decode_reg(&second_byte, &w);
    let rm: Register = Register::decode_rm(&second_byte, &w);

    if d == 1 {
        println!("mov {reg}, {rm}");
    } else {
        println!("mov {rm}, {reg}");
    }
}
