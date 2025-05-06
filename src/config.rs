use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
};

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

        let sideways = if let Some(sideways_input) = args.next() {
            sideways_input.parse().unwrap_or(false)
        } else {
            false
        };

        Ok(Config {
            folder_filepath,
            target_paths: target_path,
            amount,
            sideways,
        })
    }

    pub fn get_target_files(&self) -> Result<Vec<File>, Box<dyn Error>> {
        let mut files = vec![];

        for entry in (fs::read_dir(&self.folder_filepath)?).flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        if self
                            .target_paths
                            .iter()
                            .any(|target_path| name.starts_with(target_path))
                        {
                            if let Ok(file) = OpenOptions::new().write(true).read(true).open(path) {
                                files.push(file);
                            }
                        }
                    }
                }
            }
        }

        if files.is_empty() {
            Err("no path files match the input strings".into())
        } else {
            Ok(files)
        }
    }
}
