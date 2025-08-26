use std::{fs::File, io::Write, os::unix::fs::MetadataExt, path::PathBuf};

const BENCH_RES_DIR: &str = "bench-res";

pub fn save_cus_to_file(name: &str, compute_units_consumed: u64) {
    let mut f = File::create(
        PathBuf::from(BENCH_RES_DIR)
            .join(name)
            .with_extension("cus.txt"),
    )
    .unwrap();

    f.write_all(compute_units_consumed.to_string().as_bytes())
        .unwrap();
}

pub fn save_binsize_to_file(prog_name: &str) {
    let size = File::open(
        PathBuf::from(std::env::var("SBF_OUT_DIR").unwrap())
            .join(prog_name)
            .with_extension("so"),
    )
    .unwrap()
    .metadata()
    .unwrap()
    .size();
    File::create(
        PathBuf::from(BENCH_RES_DIR)
            .join("binsize")
            .with_extension("txt"),
    )
    .unwrap()
    .write_all(size.to_string().as_bytes())
    .unwrap();
}
