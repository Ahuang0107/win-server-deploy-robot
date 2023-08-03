use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

// use std::process::Command;
use util::*;

#[cfg(test)]
mod test;
#[allow(dead_code)]
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = tokio::fs::read_to_string("config.yaml").await?;
    let config = serde_yaml::from_str::<Config>(&content)?;

    create_dir_all("./temp")?;

    for (_, config) in config.profiles {
        let date = chrono::Utc::now().format("%F");
        let new_folder = format!("{}\\bak{date}", config.target_folder);
        let mut script_commands = vec![new_item(&new_folder)];
        for file in config.files.iter() {
            let path = Path::new(file);
            let filename = path.file_name().unwrap().to_str().unwrap();
            let from = config.target_folder.clone() + "\\" + filename;
            let to = new_folder.clone() + "\\" + filename;
            script_commands.push(move_item(&from, &to));
        }
        for file in config.files.iter() {
            let path = Path::new(file);
            let filename = path.file_name().unwrap().to_str().unwrap();
            let from = config.target_folder.clone() + "\\" + filename;
            script_commands.push(copy_item(file, &from));
        }

        let output_ps_path = format!("./temp/deploy-{}.ps1", uuid::Uuid::new_v4());
        {
            let mut output_ps_file = File::create(&output_ps_path)?;
            let output_content = vec![
                set_usr(&config.username),
                set_pwd(&config.password),
                set_crdl(),
                session(&config.host),
                invoke(&script_commands),
            ]
            .join("\n");
            println!("{output_content}");
            output_ps_file.write_all(output_content.as_bytes()).unwrap();
        }

        // let output = Command::new("powershell.exe")
        //     .arg("-File")
        //     .arg(output_ps_path)
        //     .output()
        //     .expect("Failed to execute PowerShell command.");
        // println!("{output:#?}");
    }

    Ok(())
}
