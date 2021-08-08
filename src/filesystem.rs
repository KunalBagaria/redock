pub mod functions {
    use std::fs;
    pub fn mkdir() -> std::io::Result<()> {
        fs::create_dir("icons")?;
        Ok(())
    }
    pub fn rename() -> std::io::Result<()> {
        fs::rename("icons", "icons.iconset")?;
        Ok(())
    }
    pub fn remove() -> std::io::Result<()> {
        fs::remove_dir_all("icons.iconset")?;
        Ok(())
    }
    pub fn del() -> std::io::Result<()> {
        fs::remove_file("icons.icns")?;
        Ok(())
    }
}