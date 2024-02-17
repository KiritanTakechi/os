use std::io::Write;
use std::env;
use std::fs::{self, File};
use std::io::Result;
use std::process::Command;

static TARGET_PATH: &str = "./target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rustc-link-arg=-Tkernel/script/linker.ld");
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    prepresssor().unwrap();
    insert_app_data().unwrap();
}

fn prepresssor() -> Result<()> {
    let target = env::var("TARGET").unwrap();
    let out_dir = format!("../target/{target}/release");

    let apps = fs::read_dir("../user/src/bin").unwrap()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().map(|t| t.is_file()).unwrap_or(false))
        .fold(Vec::new(), |mut acc, f| {
            acc.push(f.path().file_stem().unwrap().to_string_lossy().to_string());
            acc
        });

    apps.iter().for_each(|app| {
        let binary_path = format!("{out_dir}/{app}");
        let stripped_binary_path = format!("{out_dir}/{app}.bin");

        let status = Command::new("rust-objcopy")
        .args(["--strip-all", "-O", "binary", &binary_path, &stripped_binary_path])
        .status()
        .expect("Failed to execute rust-objcopy");

    if !status.success() {
        panic!("Failed to strip binary");
    }
    });
    
    Ok(())
}

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();

    let apps = fs::read_dir("../user/src/bin")?
        .filter_map(Result::ok)
        .filter(|f| f.file_type().map(|t| t.is_file()).unwrap_or(false))
        .fold(Vec::new(), |mut acc, f| {
            acc.push(f.path().file_stem().unwrap().to_string_lossy().to_string());
            acc
        });

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    (0..apps.len()).try_for_each(|i| -> Result<()> {
        writeln!(f, r#"    .quad app_{i}_start"#)?;
        Ok(())
    })?;

    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    apps.iter()
        .enumerate()
        .try_for_each(|(i, app)| -> Result<()> {
            println!("app_{i}: {app}");
            writeln!(
                f,
                r#"
    .section .data
    .global app_{i}_start
    .global app_{i}_end
app_{i}_start:
    .incbin "{TARGET_PATH}{app}.bin"
app_{i}_end:"#
            )?;

            Ok(())
        })?;

    Ok(())
}
