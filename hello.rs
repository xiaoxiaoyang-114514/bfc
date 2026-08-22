fn main() {
    let mut ptr = 0;
    let mut mem: Vec<u8> = vec![0];
    mem[ptr] = mem[ptr].wrapping_add(8);
    while mem[ptr] != 0 {
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        mem[ptr] = mem[ptr].wrapping_add(4);
        while mem[ptr] != 0 {
            ptr += 1;
            if ptr >= mem.len() {
                mem.push(0);
            }
            mem[ptr] = mem[ptr].wrapping_add(2);
            ptr += 1;
            if ptr >= mem.len() {
                mem.push(0);
            }
            mem[ptr] = mem[ptr].wrapping_add(3);
            ptr += 1;
            if ptr >= mem.len() {
                mem.push(0);
            }
            mem[ptr] = mem[ptr].wrapping_add(3);
            ptr += 1;
            if ptr >= mem.len() {
                mem.push(0);
            }
            mem[ptr] = mem[ptr].wrapping_add(1);
            ptr -= 1;
            if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
                mem.pop();
            }
            ptr -= 1;
            if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
                mem.pop();
            }
            ptr -= 1;
            if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
                mem.pop();
            }
            ptr -= 1;
            if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
                mem.pop();
            }
            mem[ptr] = mem[ptr].wrapping_sub(1);
        }
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        mem[ptr] = mem[ptr].wrapping_add(1);
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        mem[ptr] = mem[ptr].wrapping_sub(1);
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        ptr += 1;
        if ptr >= mem.len() {
            mem.push(0);
        }
        mem[ptr] = mem[ptr].wrapping_add(1);
        while mem[ptr] != 0 {
            ptr -= 1;
            if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
                mem.pop();
            }
        }
        ptr -= 1;
        if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
            mem.pop();
        }
        mem[ptr] = mem[ptr].wrapping_sub(1);
    }
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    print!("{}", mem[ptr] as char);
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    mem[ptr] = mem[ptr].wrapping_sub(3);
    print!("{}", mem[ptr] as char);
    mem[ptr] = mem[ptr].wrapping_add(7);
    print!("{}", mem[ptr] as char);
    print!("{}", mem[ptr] as char);
    mem[ptr] = mem[ptr].wrapping_add(3);
    print!("{}", mem[ptr] as char);
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    print!("{}", mem[ptr] as char);
    ptr -= 1;
    if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
        mem.pop();
    }
    mem[ptr] = mem[ptr].wrapping_sub(1);
    print!("{}", mem[ptr] as char);
    ptr -= 1;
    if (mem[ptr+1] == 0) && (mem.len() == ptr + 2) {
        mem.pop();
    }
    print!("{}", mem[ptr] as char);
    mem[ptr] = mem[ptr].wrapping_add(3);
    print!("{}", mem[ptr] as char);
    mem[ptr] = mem[ptr].wrapping_sub(6);
    print!("{}", mem[ptr] as char);
    mem[ptr] = mem[ptr].wrapping_sub(8);
    print!("{}", mem[ptr] as char);
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    mem[ptr] = mem[ptr].wrapping_add(1);
    print!("{}", mem[ptr] as char);
    ptr += 1;
    if ptr >= mem.len() {
        mem.push(0);
    }
    mem[ptr] = mem[ptr].wrapping_add(2);
    print!("{}", mem[ptr] as char);
}