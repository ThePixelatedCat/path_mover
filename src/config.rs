use std::{env, error, fs, path};

pub struct Config {
    pub folder_filepath: String,
    pub target_paths: Vec<String>,
    pub amount: f64,
    pub sideways: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let folder_filepath = env::var("FOLDER_FILEPATH")
            .map_err(|_err| "please set path folder filepath environment variable")?;

        let target_path = args
            .next()
            .ok_or("didn't get a target path phrase")?
            .split(',')
            .map(|str| str.to_owned())
            .collect();

        let amount = args
            .next()
            .ok_or("didn't get an amount input")?
            .parse()
            .unwrap_or(0.0);

        let sideways = args
            .next()
            .ok_or("didn't get a sideways input")?
            .parse()
            .unwrap_or(false);

        Ok(Config {
            folder_filepath,
            target_paths: target_path,
            amount,
            sideways,
        })
    }

    pub fn get_target_files(&self) -> Result<Vec<path::PathBuf>, Box<dyn error::Error>> {
        let mut file_paths = vec![];

        for result in fs::read_dir(&self.folder_filepath)? {
            let entry = result?;

            if let Ok(file_path) = entry.file_name().into_string() {
                if self
                    .target_paths
                    .iter()
                    .any(|target_path| file_path.starts_with(target_path))
                {
                    file_paths.push(entry.path());
                }
            }
        }

        Ok(file_paths)
    }
}
