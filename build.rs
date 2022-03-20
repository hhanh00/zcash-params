use zcash_primitives::constants::generate_pedersen_hash_exp_table;
use group::GroupEncoding;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::Path;

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
    if !Path::new("src/sapling-output.params").exists() {
        let home = std::env::var("HOME").unwrap();
        fs::copy(format!("{}/.zcash-params/sapling-output.params", home), "src/sapling-output.params").unwrap();
        fs::copy(format!("{}/.zcash-params/sapling-spend.params", home), "src/sapling-spend.params").unwrap();
    }
}

fn main() {
    write_generators_bin();
}
