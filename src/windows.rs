use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::printer;
use crate::process;

#[derive(Serialize, Deserialize, Debug)]
struct SystemPrinter {
    #[serde(alias = "DriverName")]
    driver_name: String,
    #[serde(alias = "Name")]
    name: String,
}

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<printer::Printer> {
    let result = process::exec(Command::new("powershell").args(["Get-Printer | ConvertTo-JSON"]));

    match result {
        Ok(dat) => {
            let printers: Vec<SystemPrinter> = serde_json::from_str(&dat).unwrap();

            printers
                .iter()
                .map(|p| printer::Printer::new(p.name.clone(), p.name.clone(), &self::print))
                .collect()
        }
        Err(err) => {
            println!("failed to get printers: {}", err);

            vec![]
        }
    }
}

/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let script = format!(
        "Get-Content -Path \"{}\" |  Out-Printer -Name \"{}\"",
        file_path, printer_system_name
    );

    let process = process::exec(Command::new("powershell").args([script]));

    if process.is_err() {
        return Result::Err(process.unwrap_err());
    }

    return Result::Ok(true);
}
