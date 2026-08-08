use std::env;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x100_0000_01b3;

fn update_hash(mut value: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        value = (value ^ u64::from(*byte)).wrapping_mul(FNV_PRIME);
    }
    value
}

fn app_names(app_dir: &Path) -> Vec<String> {
    let mut apps: Vec<_> = fs::read_dir(app_dir)
        .expect("failed to read user application directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("rs")))
        .map(|path| {
            path.file_stem()
                .expect("application source has no file stem")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    apps.sort();
    apps
}

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let user_dir = manifest_dir.join("../user");
    let app_dir = user_dir.join("src/bin");
    let user_target_dir = user_dir.join("target").join(target).join(profile);
    let linker_source = manifest_dir.join("src/linker.ld");

    println!("cargo::rerun-if-changed={}", app_dir.display());
    println!("cargo::rerun-if-changed={}", linker_source.display());

    let linker_script = fs::read(&linker_source).expect("failed to read kernel linker script");
    let linker_hash = update_hash(FNV_OFFSET, &linker_script);
    let linker_path = out_dir.join(format!("linker-{linker_hash:016x}.ld"));
    fs::write(&linker_path, linker_script).expect("failed to write kernel linker script");
    println!("cargo::rustc-link-arg-bin=os=-T{}", linker_path.display());

    let apps = app_names(&app_dir);
    let mut binaries = Vec::with_capacity(apps.len());
    let mut bundle_hash = FNV_OFFSET;
    let mut missing_binaries = false;

    for app in &apps {
        let binary_path = user_target_dir.join(format!("{app}.bin"));
        println!("cargo::rerun-if-changed={}", binary_path.display());
        bundle_hash = update_hash(bundle_hash, app.as_bytes());
        match fs::read(&binary_path) {
            Ok(bytes) => bundle_hash = update_hash(bundle_hash, &bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                // `cargo check` (and therefore rust-analyzer) does not assemble
                // the `.incbin` directives below. Keep emitting them so a real
                // build still reports a missing user image instead of silently
                // producing an unusable kernel.
                bundle_hash = update_hash(bundle_hash, binary_path.as_os_str().as_encoded_bytes());
                missing_binaries = true;
            }
            Err(error) => panic!(
                "failed to read user binary '{}': {error}",
                binary_path.display()
            ),
        }
        binaries.push((app, binary_path));
    }

    if missing_binaries {
        println!(
            "cargo::warning=user binaries are missing; `cargo check` can continue, but run `make user` before building the kernel"
        );
    }

    let mut link_app = format!(
        ".align 3\n.section .data\n.global _num_app\n_num_app:\n    .quad {}\n",
        apps.len()
    );
    for index in 0..apps.len() {
        writeln!(link_app, "    .quad app_{index}_start").unwrap();
    }
    if let Some(last) = apps.len().checked_sub(1) {
        writeln!(link_app, "    .quad app_{last}_end").unwrap();
    }

    for (index, (_, binary_path)) in binaries.iter().enumerate() {
        writeln!(
            link_app,
            r#".section .data
.global app_{index}_start
.global app_{index}_end
app_{index}_start:
    .incbin "{}"
app_{index}_end:
"#,
            binary_path.display()
        )
        .unwrap();
    }

    let link_app_path = out_dir.join(format!("link_app-{bundle_hash:016x}.S"));
    fs::write(&link_app_path, link_app).expect("failed to write application assembly table");
    println!(
        "cargo::rustc-env=OS_LINK_APP_ASM={}",
        link_app_path.display()
    );
}
