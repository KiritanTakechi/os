fn main() {
    let target_arch = match std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or("riscv64".to_string())
        .as_str()
    {
        "riscv32" | "riscv64" => "riscv".to_owned(),
        "x86_64" => "x86".to_owned(),
        target_arch => panic!("Unsupported target architecture: {}", target_arch),
    };

    println!("cargo:rustc-link-arg=-Tkernel/src/arch/{target_arch}/boot/linker.ld");
}
