#[cfg(test)]
mod tests {
    use duct::cmd;
    use duct_sh::sh;
    use std::path::Path;
    #[test]
    fn test_cat() {
        let path = Path::new("/opt/homebrew/README.md");
        let result = cmd!("cat", path).read();

        match result {
            Ok(readme) => println!("{:?}", readme),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_pipe() {
        let result = sh("yes | head -5").pipe(cmd!("xargs", "echo")).read();

        match result {
            Ok(ok) => println!("{:?}", ok),
            Err(e) => println!("{:?}", e),
        }
    }
}
