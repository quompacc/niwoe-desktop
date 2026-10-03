use super::DesktopApp;

impl DesktopApp {
    /// Local files only, through an explicit standalone desktop Exec %f/%F slot.
    /// There is no shell interpolation, generic argument append or title guessing.
    pub fn restore_argv(&self, file: Option<&str>) -> Result<(String, Vec<String>), String> {
        if !super::is_executable_available(&self.program) {
            return Err("App ist nicht mehr installiert".into());
        }
        let Some(file) = file else {
            return Ok((self.program.clone(), self.args.clone()));
        };
        let path = std::path::Path::new(file);
        if !path.is_absolute() || !path.is_file() {
            return Err("Datei fehlt oder ist kein absoluter Dateipfad".into());
        }
        let argv = self
            .file_argv
            .as_ref()
            .ok_or("App unterstützt keinen sicheren lokalen Dateiaufruf (%f/%F)")?;
        let mut expanded = argv.iter().map(|s| {
            if s == "%f" || s == "%F" {
                file.to_string()
            } else {
                s.clone()
            }
        });
        Ok((
            expanded.next().ok_or("App-Aufruf fehlt")?,
            expanded.collect(),
        ))
    }
}
