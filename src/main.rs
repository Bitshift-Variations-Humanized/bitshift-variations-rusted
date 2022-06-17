use rusted_variations as variations;

fn main() -> std::io::Result<()> {
    let v : variations::Variations<u8> = Default::default();
    let song = v.collect::<Vec<_>>();
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut stdout_locked = stdout.lock();
    stdout_locked.write(&song)?;
    stdout_locked.flush()?;
    Ok(())
}
