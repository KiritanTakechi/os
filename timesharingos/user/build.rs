fn main() {
    println!("cargo:rustc-link-arg=-Tuser/script/linker.ld");
}

// fn build_bin() -> Result<()> {
//     let bin_name = std::env::var("CARGO_PKG_NAME").unwrap();

//     let app_source = fs::read_dir("user/src/bin")?
//         .filter_map(Result::ok)
//         .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
//         .enumerate()
//         .map(|(index, entry)| (entry.file_name().to_string_lossy().to_string(), index))
//         .collect::<HashMap<String, usize>>();

//     Ok(())
// }
