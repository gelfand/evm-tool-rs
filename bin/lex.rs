use evm_tool::yul::lexer;

fn main() {
    let input = r#"
object "Contract" {
    code {
        datacopy(0, dataoffset("runtime"), datasize("runtime"))
        return(0, datasize("runtime"))
    }
    object "runtime" {
        code {
            switch shr(0xf8, calldataload(0))
            // calldata shifted by 248 bits to the right
            // is equivalent of the byte slice calldata[0:1]
            case 0x00 {
                mstore(0, "Hello, World")
                return(0, 0x20)
            }
            default { 
                revert(0, 0)
            }
        }
    }
}
"#;
    let lexed = lexer::parse(input);
    dbg!(lexed);
}
