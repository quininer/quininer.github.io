use std::{ io, fs };
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    let dist = Path::new("dist");

    if !dist.exists() {
        fs::create_dir(dist)?;
    }
    
    for entry in fs::read_dir("posts")? {
        let entry = entry?;
        let input = entry.path();

        if input.extension() != Some(OsStr::new("typst")) {
            continue
        }

        let mut output = dist.join(input.file_name().unwrap());
        output.set_extension("html");

        let mut cmd = Command::new("typst");
        cmd
            .arg("compile")
            .args(["--root", "."])
            .args(["--features", "html"])
            .args(["--format", "html"])
            .args([&input, &output]);

        println!("{:?}", cmd);

        let status = cmd.status()?;
        if !status.success() {
            panic!("typst compile failed: {:?}", input)
        }
    }

    Ok(())
}
