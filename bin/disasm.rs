use evm_tool::disasm::Opcode;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "i", long = "input")]
    input: String,
}

fn main() {
    let args = Opts::from_args();

    let mut input = args.input;
    if input.starts_with("0x") {
        input = input.replace("0x", "")
    }

    let mut bytes = Vec::new();
    for i in 0..input.len() / 2 {
        let byte = u8::from_str_radix(&input[i * 2..i * 2 + 2], 16).unwrap();
        bytes.push(byte);
    }

    println!("{:?}", Opcode::disasm(bytes));
}
