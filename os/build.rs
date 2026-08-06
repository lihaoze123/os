use std::fs::{File, read_dir};
use std::io::Write;

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);

    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            dir_entry
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}
"#,
        apps.len()
    )
    .unwrap();

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i).unwrap();
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1).unwrap();

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )
        .unwrap();
    }
}
