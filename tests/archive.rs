mod common;

test_format!(
    Archive,
    "application/vnd.bzip3",
    "bz3",
    bz3,
    "sample.tar.bz3"
);
test_format!(
    Archive,
    "application/vnd.sqlite3",
    "sqlite",
    sqlite,
    "sample.db"
);

test_format!(Archive, "application/zstd", "zst", zst, "sample.tar.zst");
test_format!(Archive, "application/x-lz4", "lz4", lz4, "sample.tar.lz4");
test_format!(Archive, "application/x-cpio", "cpio", cpio, "sample.cpio");
test_format!(
    Archive,
    "application/zstd",
    "zst",
    zst_skip,
    "sample.skippable.zst"
);
test_format!(Archive, "application/x-par2", "par2", par2, "sample.par2");
