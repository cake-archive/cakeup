use super::CakeupResult;

pub enum Platform {
    Windows,
    Linux,
    MacOS
}

pub fn get_platform_name() -> CakeupResult<String> {
    return match get_platform() {
        Ok(Platform::Windows) => Ok(String::from("windows")),
        Ok(Platform::Linux) => Ok(String::from("linux")),
        Ok(Platform::MacOS) => Ok(String::from("macos")),
        Err(e) => Err(e)
    };
}

pub fn get_platform() -> CakeupResult<Platform> {
    if cfg!(target_os = "windows") {
        return Ok(Platform::Windows)
    } else if cfg!(target_os = "linux") {
        return Ok(Platform::Linux)
    } else if cfg!(target_os = "macos") {
        return Ok(Platform::MacOS)
    }
    return Err(format_err!("Could not get platform."));
}