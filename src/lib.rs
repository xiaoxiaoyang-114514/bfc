pub fn bfrs(src: &str) -> String {
    let src_bytes = src.as_bytes();
    let mut rscode: Vec<String> = Vec::new();
    if src_bytes.contains(&44) {
        rscode.push("use std::io::Read;".to_string());
    }
    let mut floor: usize = 1;
    rscode.push("fn main() {".to_string());
    rscode.push("    let mut ptr = 0;".to_string());
    rscode.push("    let mut mem: Vec<u8> = vec![0];".to_string());
    if src_bytes.contains(&44) {
        rscode.push("    let mut input = std::io::stdin().bytes();".to_string());
    }
    let mut temp: isize = 0;
    let mut flag: bool = false;
    for c in src_bytes {
        match c {
            62 => {
                flag = true;
                rscode.push(format!("{}ptr += 1;"," ".repeat(floor * 4)).to_string());
                rscode.push(format!("{0}if ptr >= mem.len() {{\n{0}    mem.push(0);\n{0}}}", " ".repeat(floor * 4)).to_string());
            }
            60 => {
                flag = true;
                rscode.push(
                    format!("{0}if ptr == 0 {{\n{0}    eprintln!(\"{{}}\", \"the index is negative.\".to_string());\n{0}    std::process::exit(1);\n{0}}};", " ".repeat(floor * 4))
                        .to_string(),
                );
                rscode.push(
                    format!("{0}ptr -= 1;\n{0}if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {{\n{0}    mem.pop();\n{0}}}", " ".repeat(floor * 4))
                        .to_string(),
                );
            }
            43 => {
                temp += 1;
            }
            45 => {
                temp -= 1;
            }
            46 => {
                flag = true;
                rscode.push(format!("{0}print!(\"{{}}\", mem[ptr] as char);", " ".repeat(floor * 4)).to_string());
            }
            44 => {
                flag = true;
                rscode
                    .push(format!("{0}mem [ptr] = match input.next() {{{0}    Some(Ok(c)) => c,\n{0}    _ => 0,{0}}};", " ".repeat(floor * 4)).to_string());
            }
            91 => {
                flag = true;
                rscode.push(format!("{0}while mem[ptr] != 0 {{", " ".repeat(floor * 4)).to_string());
                floor += 1;
            }
            93 => {
                flag = true;
                floor -= 1;
                rscode.push(format!("{0}}}", " ".repeat(floor * 4)).to_string());
            }
            _ => {
                flag = true;
            }
        }

        if flag {
            if temp > 0 {
                rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
            } else if temp < 0 {
                rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
            }
            flag = false;
        }

    }

    rscode.push("}".to_string());
    let code = rscode.join("\n");
    return code.clone();
}
