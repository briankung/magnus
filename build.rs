use std::{env::{self, VarError}, ffi::OsStr, process::Command};

const RUBY_VERSIONS: [(u8, u8); 3] = [(2, 7), (3, 0), (3, 1)];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=RUBY");
    println!("cargo:rerun-if-env-changed=RBCONFIG_RUBY_API_VERSION");

    let version = match env::var("RBCONFIG_RUBY_API_VERSION") {
        Ok(v) => v,
        Err(VarError::NotPresent) => {
            let ruby = match env::var_os("RUBY") {
                Some(val) => val,
                None => OsStr::new("ruby").to_os_string(),
            };
            let output = Command::new(ruby)
                .arg("-e")
                .arg("print RbConfig::CONFIG[\"RUBY_API_VERSION\"]")
                .output()?;
            String::from_utf8(output.stdout)?
        }
        Err(e) => return Err(e.into()),
    };

    let version_parts = version
        .split('.')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;
    let version = (version_parts[0], version_parts[1]);
    for &v in &RUBY_VERSIONS {
        if version < v {
            println!(r#"cargo:rustc-cfg=ruby_lt_{}_{}"#, v.0, v.1);
        }
        if version <= v {
            println!(r#"cargo:rustc-cfg=ruby_lte_{}_{}"#, v.0, v.1);
        }
        if version == v {
            println!(r#"cargo:rustc-cfg=ruby_{}_{}"#, v.0, v.1);
        }
        if version >= v {
            println!(r#"cargo:rustc-cfg=ruby_gte_{}_{}"#, v.0, v.1);
        }
        if version > v {
            println!(r#"cargo:rustc-cfg=ruby_gt_{}_{}"#, v.0, v.1);
        }
    }
    Ok(())
}
