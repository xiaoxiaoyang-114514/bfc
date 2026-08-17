pub fn bfrs(src: &str) -> String {
    let src_bytes = src.as_bytes();
    let mut rscode: Vec<String> = Vec::new();
    if src_bytes.contains(&44) {
        rscode.push("use std::io::Read;".to_string());
    }
    rscode.push("fn main() {".to_string());
    rscode.push("let mut ptr = 0;".to_string());
    rscode.push("let mut mem: Vec<u8> = vec![0];".to_string());
    if src_bytes.contains(&44) {
        rscode.push("let mut input = std::io::stdin().bytes();".to_string());
    }


    let mut line = 0;
    let mut c: u8;
    while line < src_bytes.len(){
        c = src_bytes[line];
        if c == 62 {
            rscode.push("ptr += 1;".to_string());
            rscode.push("if ptr >= mem.len() {mem.push(0);}".to_string());
        } else if c == 60 {
            rscode.push("if ptr == 0{panic!(\"{}\",\"the index is negative.\".to_string());}".to_string());
            rscode.push("ptr -= 1;if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {mem.pop();}".to_string());
        } else if c == 43 {
            rscode.push("mem[ptr] = mem[ptr].wrapping_add(1);".to_string());
        } else if c == 45 {
            rscode.push("mem[ptr] = mem[ptr].wrapping_sub(1);".to_string());
        } else if c == 46 {
            rscode.push("print!(\"{}\",mem[ptr] as char);".to_string());
        } else if c == 44 {
            rscode.push("mem [ptr] = match input.next() {Some(Ok(c)) => c,_ => 0,};".to_string());
        } else if c == 91 {
             rscode.push("while mem[ptr] != 0{".to_string());
        } else if c == 93 {
             rscode.push("}".to_string());
        }
        line += 1;
    }

    rscode.push("}".to_string());
    let code = rscode.join("\n");
    return code.clone();


}

