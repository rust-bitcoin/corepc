#!/bin/bash
set -e
set -x

# Regenerate targets to ensure they're up to date
pushd src/bin
rm -f *_target.rs
./gen_target.sh
[ "$(git diff)" != "" ] && exit 1
popd

export RUSTFLAGS="--cfg=fuzzing"

cargo install --color always --force honggfuzz --no-default-features

# Because we're fuzzing relatively few iterations, the maximum possible
# compiler optimizations aren't necessary, so we turn off LTO
sed -i 's/lto = true//' Cargo.toml

export HFUZZ_BUILD_ARGS="--features honggfuzz_fuzz"

cargo --color always hfuzz build -j8
for TARGET in src/bin/*_target.rs; do
	FILENAME=$(basename $TARGET)
	FILE="${FILENAME%.*}"
	HFUZZ_RUN_ARGS="--exit_upon_crash -v -n8 --run_time 30"
	export HFUZZ_RUN_ARGS
	cargo --color always hfuzz run $FILE
	if [ -f hfuzz_workspace/$FILE/HONGGFUZZ.REPORT.TXT ]; then
		cat hfuzz_workspace/$FILE/HONGGFUZZ.REPORT.TXT
		for CASE in hfuzz_workspace/$FILE/SIG*; do
			cat $CASE | xxd -p
		done
		exit 1
	fi
done
