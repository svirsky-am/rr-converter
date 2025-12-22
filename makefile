.PHONY: run-test-of-libs
run-test-of-libs:
	rm -rf rr-parser-lib/output
	cargo test -p rr-parser-lib -- --show-output 
# RUST_BACKTRACE=full cargo test -p rr-parser-lib

.PHONY: run-test-of-libs-one-shot
run-test-of-libs-one-shot:
	rm -rf rr-parser-lib/output
	cargo test -p rr-parser-lib  -- --show-output parser::tests::tests::test_convert_csv_to_xml_via_trait  


.PHONY: fmt-libs
fmt-libs:
	cargo fmt -p rr-parser-lib
	cargo clippy --allow-dirty --fix -p rr-parser-lib

.PHONY: run-fix-lint-for-lib
run-fix-lint-for-lib:
	cargo fix --lib -p rr-parser-lib
	cargo fix --lib -p rr-parser-lib --tests

.PHONY: run-test-of-bin
run-test-of-bin:
	cargo test -p rr-file-processor

.PHONY: sandbox-env
sandbox-env:
	cargo run -p sandbox_env


.PHONY: build-and-exec-args-mode
build-and-exec-args-mode: run-test-of-libs
	cargo build -p rr-file-processor
	cargo run -p rr-file-processor -- \
		--in-format csv_extra_fin --out-format yaml \
		--input tests/test_files/example_of_report_bill_1.csv \
		--output output/formatted/result.xml

.PHONY: test_stdin_csv_to_xml
test_stdin_csv_to_xml:
	cat tests/test_files/example_of_report_bill_1.csv  | \
		target/debug/rr-file-processor \
			--in-format csv_extra_fin --out-format yaml \
			--input  - \
			--output output/formatted/stdin_csv_to_xml

.PHONY: test_csv_to_xml
test_csv_to_xml:
	target/debug/rr-file-processor \
		--in-format csv_extra_fin --out-format camt_053 \
		--input  tests/test_files/example_of_report_bill_1.csv  \
		--output output/formatted/csv_to_xml

.PHONY: test_xml_to_csv
test_xml_to_csv:
	target/debug/rr-file-processor \
			--in-format camt_053 --out-format csv_extra_fin \
			--input  tests/test_files/camt_053_treasurease.xml \
			--output output/formatted/xml_to_csv

.PHONY: test-csv-to-xml-payload
test-csv-to-xml-payload:
	target/debug/rr-file-processor \
		--in-format csv_extra_fin 
		--out-format xml \
		--input  tests/test_files/example_of_report_bill_1.csv  \
		--output output/payload/csv-to-xml


.PHONY: clean-run
#TODO^ fixup test_stdin_csv_to_xml
test-clean-run:  test_csv_to_xml test_xml_to_csv
	echo condvert 

# .PHONY: build-and-exec
# build-and-exec:
# 	cargo build -p rr-file-processor
# 	cargo run -p rr-file-processor -- tests/test_files/hello.txt tests/test_files/rust.txt


.PHONY: linting
linting:
	cargo fmt
	cargo clippy
	cargo check


.PHONY: all
all: run-test-of-libs run-test-of-bin build-and-exec-args-mode