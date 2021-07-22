use zcash_primitives::constants::generate_pedersen_hash_exp_table;
use group::GroupEncoding;
use std::io::Write;
use std::fs::File;

#[allow(dead_code)]
fn write_generators_bin() {
    let hash_exp_table = generate_pedersen_hash_exp_table();

    let mut bb: Vec<u8> = vec![];
    for g in hash_exp_table.iter().take(3) {
        for w in g.iter() {
            for p in w.iter() {
                let h = p.to_bytes();
                bb.write(&h).unwrap();
            }
        }
    }
    let mut f = File::create("src/generators.bin").unwrap();
    f.write(&bb).unwrap();
}

fn main() {
    write_generators_bin();
}
