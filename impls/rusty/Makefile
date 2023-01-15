# UPPER_STEPS = step4_if_fn_do step5_tco step6_file step7_quote step8_macros step9_try stepA_mal
STEPS = step0_repl step1_read_print

all: $(STEPS)

dist: mal

mal: stepA_mal
	cp $< $@

%: src/bin/%.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml

step0_repl: $(STEP0_DEPS)
step1_read_print: $(STEP0_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
