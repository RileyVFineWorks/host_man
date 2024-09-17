use std::io;
use winres::WindowsResource;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            // This path is relative to your project root
            .set_icon("assets/host_man.ico")
            .compile()?;
    }
    Ok(())
}