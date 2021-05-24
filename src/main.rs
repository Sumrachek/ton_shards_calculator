fn add_childs(parent: u64, depth: u32, max_depth: u32, columns: &mut Vec<Vec<u64>>) {
    let c = parent & (0xffff_ffff_ffff_ffff_u64.checked_shl(64 - depth + 1).unwrap_or(0));

    let c1 = c | (0x8000_0000_0000_0000 >> depth);
    columns[depth as usize].push(c1);
    if depth < max_depth {
        add_childs(c1, depth + 1, max_depth, columns);
    }

    let c2 = c | (0xc000_0000_0000_0000 >> (depth - 1));
    columns[depth as usize].push(c2);
    if depth < max_depth {
        add_childs(c2, depth + 1, max_depth, columns);
    }
}

fn print_shardes_tree(max_depth: u32) {
    let mut columns = vec!();
    for d in 0..=max_depth {
        columns.push(vec!());
    }
    let root = 0x8000_0000_0000_0000;
    columns[0].push(root);
    add_childs(root, 1, max_depth, &mut columns);
    
    let total_s = 2_usize.pow(max_depth);
    for row in 0..total_s {
        for col in &columns {
            if row % (total_s / col.len()) == 0 {
                let v = col[row / (total_s / col.len() )] >> 48;
                print!("{:04x}({:016b})", v, v);
            } else {
                print!("                      ");
            }
            print!("  ");
        }
        print!("\n");
    }
}

fn main() {
    print_shardes_tree(5);
}
