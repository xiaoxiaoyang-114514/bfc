pub fn bfrs(src: &str) -> Result<String, String> {
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
    let mut temp_ptr = 0;
    for c in src_bytes {
        match c {
            62 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                temp_ptr += 1;
                rscode.push(format!("{}ptr += 1;"," ".repeat(floor * 4)).to_string());
                rscode.push(format!("{0}if ptr >= mem.len() {{\n{0}    mem.push(0);\n{0}}}", " ".repeat(floor * 4)).to_string());
            }
            60 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                temp_ptr -= 1;
                if temp_ptr < 0 {
                    return Err("The index is negative.".to_string());
                }
                rscode.push(
                    format!("{0}ptr -= 1;\n{0}if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {{\n{0}    mem.pop();\n{0}}}", " ".repeat(floor * 4))
                        .to_string(),
                );
            },
            43 => temp += 1,
            45 => temp -= 1,
            46 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                rscode.push(format!("{0}print!(\"{{}}\", mem[ptr] as char);", " ".repeat(floor * 4)).to_string());
            },
            44 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                rscode
                    .push(format!("{0}mem [ptr] = match input.next() {{{0}    Some(Ok(c)) => c,\n{0}    _ => 0,{0}}};", " ".repeat(floor * 4)).to_string());
            },
            91 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                rscode.push(format!("{0}while mem[ptr] != 0 {{", " ".repeat(floor * 4)).to_string());
                floor += 1;
            },
            93 => {
                if temp > 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_add({});", " ".repeat(floor * 4), temp).to_string());
                } else if temp < 0 {
                    rscode.push(format!("{}mem[ptr] = mem[ptr].wrapping_sub({});", " ".repeat(floor * 4), 0 - temp).to_string());
                }
                temp = 0;
                floor -= 1;
                rscode.push(format!("{0}}}", " ".repeat(floor * 4)).to_string());
            },
            _ => {},
        }


    }

    rscode.push("}".to_string());
    let code = rscode.join("\n");
    return Ok(code);
}


