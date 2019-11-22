extern crate rand;

use rand::Rng;

const ROW: usize = 5;
const COLUMN: usize = 6;
const ORB_COLORS: u32 = 5;
const MATCH_ORBS: usize = 3;

type Table = [[u32; COLUMN]; ROW];

fn generate_orb() -> u32 {
    rand::thread_rng().gen_range(1, ORB_COLORS + 1)
}

fn generate_table(table: &mut Table) {
    for row in table {
        for orb in row {
            *orb = generate_orb();
        }
    }
}

fn is_match(table: Table) -> bool {

    // row match check
    for i in 0..ROW {
        for j in 0..COLUMN - MATCH_ORBS + 1 {
            let first = &table[i][j];
            let mut is_all_numbers_same = true;
            for k in j..MATCH_ORBS + j {
                if table[i][k] != *first {
                    is_all_numbers_same = false;
                }
            }
            if is_all_numbers_same {
                return true;
            }
        }
    }

    // column match check
    for i in 0..ROW - MATCH_ORBS + 1 {
        for j in 0..COLUMN {
            let first = &table[i][j];
            let mut is_all_numbers_same = true;
            for k in i..MATCH_ORBS + i {
                if table[k][j] != *first {
                    is_all_numbers_same = false;
                }
            }
            if is_all_numbers_same {
                return true;
            }
        }
    }

    false
}


fn main() {
    let mut table: Table = [[0; COLUMN]; ROW];
    generate_table(&mut table);

    while is_match(table) {
        generate_table(&mut table);
    }

    for row in &table {
        println!("{:?}", row);
    }
}
