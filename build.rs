use std::process::{Command, Stdio};

fn main() -> Result<(), String> {
    let output = Command::new("fortune")
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to run `fortune`: {:?}", e))?;

    if !output.status.success() {
        return Err(format!("`fortune` exited with an error..."));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!(
        "cargo:rustc-env=MY_FORTUNE={}",
        stdout
    );

    Ok(())
}

