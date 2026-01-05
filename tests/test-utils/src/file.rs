use std::{fs::File, os::unix::fs::MetadataExt, path::PathBuf};

use expect_test::Expect;

pub fn bench_binsize(prog_name: &str, expect: Expect) {
    let size = File::open(
        PathBuf::from(std::env::var("SBF_OUT_DIR").unwrap())
            .join(prog_name)
            .with_extension("so"),
    )
    .unwrap()
    .metadata()
    .unwrap()
    .size();
    expect.assert_eq(&size.to_string());
}
