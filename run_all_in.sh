#!/bin/bash

set -ex;

CUR_DIR=$(dirname $0)

pushd $CUR_DIR ;
    cargo fmt;
    cargo clippy ;
    cargo check ;
    cargo test -p rr-parser-lib;
    cargo test -p rr-file-processor;
    # Build
    cargo build -p rr-file-processor
    # Execute with test files
    cargo run -p rr-file-processor -- --input rr-file-processor/tests/test_files/data.csv --in-format csv \
		--output output/formatted/result.xml --out-format xml

    # cargo run --bin utils
popd;