use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_ADDRESS: usize = 0x8040_0000;
const ADDRESS_STEP: usize = 0x2_0000;

fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |value, byte| {
        (value ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
    })
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
    let app_dir = manifest_dir.join("src/bin");
    let linker_template_path = manifest_dir.join("src/linker.ld.in");
    let linker_template = fs::read_to_string(&linker_template_path)
        .expect("failed to read user linker script template");

    println!("cargo::rerun-if-changed={}", app_dir.display());
    println!("cargo::rerun-if-changed={}", linker_template_path.display());

    for (index, app) in app_names(&app_dir).iter().enumerate() {
        let address = BASE_ADDRESS + ADDRESS_STEP * index;
        let linker_script = linker_template.replace("@BASE_ADDRESS@", &format!("{address:#x}"));
        assert_ne!(
            linker_script, linker_template,
            "user linker script template is missing @BASE_ADDRESS@"
        );

        let linker_path = out_dir.join(format!(
            "linker-{app}-{:016x}.ld",
            hash(linker_script.as_bytes())
        ));
        fs::write(&linker_path, linker_script).expect("failed to write user linker script");

        println!(
            "cargo::rustc-link-arg-bin={app}=-T{}",
            linker_path.display()
        );
    }
}
