pub fn run_command(cmd: &str) -> Result<String, String> {
    use std::process::Command;
    let command = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env("MYVAR_AUTHOR", "0xAF")
        .output()
        .expect("failed to execute process");
    if command.status.success() {
        let stdout = command.stdout.to_vec();
        Ok(std::str::from_utf8(&stdout).unwrap().to_string())
    } else {
        let stderr = command.stderr.to_vec();
        Ok(std::str::from_utf8(&stderr).unwrap().to_string())
    }
}
