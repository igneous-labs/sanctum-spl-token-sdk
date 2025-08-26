use std::{fs::File, io::Write, path::PathBuf};

pub fn save_cus_to_file(name: &str, compute_units_consumed: u64) {
    let mut f = File::create(PathBuf::from("bench-cus").join(name).with_extension("txt")).unwrap();

    f.write_all(compute_units_consumed.to_string().as_bytes())
        .unwrap();
}
