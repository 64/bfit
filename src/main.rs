#[derive(Copy, Clone)]
enum Op {
    Add(i8),
    Move(isize),
    Input,
    Output,
    LoopBegin,
    LoopEnd,
}

fn interpret(op_list: &[Op], jump_table: &[usize]) {
    let mut memory = vec![0i8; 30000];
    let mut data_ptr = 0usize;
    let mut op_ptr = 0usize;

    while op_ptr < op_list.len() {
        match op_list[op_ptr] {
            Op::Add(x) => {
                memory[data_ptr] = memory[data_ptr].wrapping_add(x);
                op_ptr += 1;
            }
            Op::Move(x) => {
                data_ptr = data_ptr.wrapping_add(x as usize);
                op_ptr += 1;
            }
            Op::LoopBegin => {
                if memory[data_ptr] == 0 {
                    op_ptr = jump_table[op_ptr];
                } else {
                    op_ptr += 1;
                }
            }
            Op::LoopEnd => {
                if memory[data_ptr] != 0 {
                    op_ptr = jump_table[op_ptr];
                } else {
                    op_ptr += 1;
                }
            }
            Op::Output => {
                #[cfg(not(target_os = "windows"))]
                unsafe { libc::putchar_unlocked(memory[data_ptr] as i32); }

                #[cfg(target_os = "windows")]
                unsafe { libc::putchar(memory[data_ptr] as i32); }

                op_ptr += 1;
            }
            Op::Input => {
                memory[data_ptr] = unsafe { libc::getchar() as i8 };
                op_ptr += 1;
            }
        }
    }
}

pub fn main() {
    let source = std::env::args().nth(1).expect("no program source");
    let mut op_list = Vec::with_capacity(source.len());

    for byte in source.bytes() {
        let op = match byte {
            b'+' => Op::Add(1),
            b'-' => Op::Add(-1),
            b'>' => Op::Move(1),
            b'<' => Op::Move(-1),
            b',' => Op::Input,
            b'.' => Op::Output,
            b'[' => Op::LoopBegin,
            b']' => Op::LoopEnd,
            _ => continue,
        };

        match (op_list.last_mut(), op) {
            (Some(Op::Add(p)), Op::Add(x)) => {
                *p = p.wrapping_add(x);
            }
            (Some(Op::Move(p)), Op::Move(x)) => {
                *p += x;
            }
            _ => op_list.push(op),
        }
    }

    let mut jump_table = vec![0; op_list.len()];
    let mut loop_begin_stack = Vec::with_capacity(op_list.len());

    for (i, op) in op_list.iter().enumerate() {
        match op {
            Op::LoopBegin => loop_begin_stack.push(i),
            Op::LoopEnd => {
                let popped = loop_begin_stack.pop().expect("unmatching loop brace");
                jump_table[popped] = i + 1;
                jump_table[i] = popped + 1;
            }
            _ => (),
        }
    }

    interpret(&op_list, &jump_table);
}