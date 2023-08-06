use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str;
use uuid::Uuid;

/// Printer is a struct to representation the system printer
/// They has an ID composed by your system_name and has printing method to print directly
#[derive(Debug)]
pub struct Printer {
    /// Visual reference of system printer name
    pub name: String,
    /// Name of Printer exactly as on system
    pub system_name: String,
    /// Name of printer driver on system
    pub driver_name: String,
}

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Get printers on windows using wmic
#[cfg(target_os = "windows")]
pub fn get_printers() -> Vec<Printer> {
    let command = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Printer | Format-List Name,DriverName")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .unwrap();

    if command.status.success() {
        let out_str = str::from_utf8(&command.stdout).unwrap();
        let lines: Vec<Vec<&str>> = out_str
            .trim()
            .split("\r\n\r\n")
            .map(|l| l.split("\r\n").collect())
            .collect();

        let mut printers: Vec<Printer> = Vec::with_capacity(lines.len());

        for line in lines {
            let name = line[0].split(":").last().unwrap().trim();
            let driver_name = line[1].split(":").last().unwrap().trim();

            printers.push(Printer {
                name: name.to_string(),
                system_name: name.to_string(),
                driver_name: driver_name.to_string(),
            });
        }

        return printers;
    }

    return Vec::with_capacity(0);
}

#[cfg(not(target_os = "windows"))]
pub fn get_printers() -> Vec<Printer> {
    let command = Command::new("lpstat").arg("-e").output().unwrap();

    if command.status.success() {
        let out_str = str::from_utf8(&command.stdout).unwrap();
        let lines: Vec<&str> = out_str.split_inclusive("\n").collect();
        let mut printers: Vec<Printer> = Vec::with_capacity(lines.len());

        for line in lines {
            let system_name = line.replace("\n", "");
            let name = String::from(system_name.replace("_", " ").trim());

            printers.push(Printer {
                name,
                system_name,
                driver_name: "".to_string(),
            });
        }

        return printers;
    }

    return Vec::with_capacity(0);
}

#[cfg(not(target_os = "windows"))]
pub fn print_file(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let command = Command::new("lp")
        .arg("-d")
        .arg(printer_system_name)
        .arg(file_path)
        .output()
        .unwrap();

    if command.status.success() {
        return Ok(true);
    }

    return Err(str::from_utf8(&command.stderr).unwrap().to_string());
}

#[cfg(target_os = "windows")]
pub fn print_file(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let child = Command::new("powershell")
        .arg(format!(
            "Get-Content -Path \"{}\" |  Out-Printer -Name \"{}\"",
            file_path, printer_system_name
        ))
        .spawn()
        .unwrap();

    if child.id() > 0 {
        return Ok(true);
    }

    return Err("Failure to start print process".to_string());
}

/// Print bytes with self printer instnace
pub fn print(printer_system_name: &str, buffer: &[u8]) -> Result<bool, String> {
    let tmp_file_path = env::temp_dir().join(Uuid::new_v4().to_string());

    let mut tmp_file = File::create(&tmp_file_path).unwrap();
    let save = tmp_file.write(buffer);

    if save.is_err() {
        return Err(save.err().unwrap().to_string());
    }

    return print_file(
        printer_system_name,
        tmp_file_path.as_os_str().to_str().unwrap(),
    );
}
