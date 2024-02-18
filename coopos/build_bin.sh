#!/bin/bash

file_counter=0

for file in user/src/bin/*; do
    filename=$(basename "$file")
    binname="${filename%.*}"

    new_address=$(printf "0x%x" $((0x80400000 + 0x20000 * file_counter)))

    sed -i "s/USER_BASE_ADDRESS = 0x[0-9a-fA-F]*;/USER_BASE_ADDRESS = $new_address;/g" user/script/linker.ld

    cargo build --release --bin "$binname"

    rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/$binname -O binary target/riscv64gc-unknown-none-elf/release/$binname.bin

    file_counter=$((file_counter + 1))
done

sed -i 's/USER_BASE_ADDRESS = 0x[0-9a-fA-F]*;/USER_BASE_ADDRESS = 0x80400000;/g' user/script/linker.ld