use std::{
    error,
    fs::{self, OpenOptions},
    path,
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

        let folder_filepath = "src\\main\\deploy\\pathplanner\\paths".to_owned();

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

    pub fn get_target_files(&self) -> Result<Vec<fs::File>, Box<dyn error::Error>> {
        let files: Vec<fs::File> = fs::read_dir(&self.folder_filepath)?
            .map(|path| path.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| {
                self.target_paths
                    .iter()
                    .any(|target_path| pathbuf_filename(path).starts_with(target_path))
            })
            .map(|path| {
                OpenOptions::new()
                    .write(true)
                    .read(true)
                    .open(path)
                    .unwrap()
            })
            .collect();
 
        if files.len() > 0 {
            Ok(files)
        } else {
            Err("no path files match the input strings".into())
        }

    }
}

fn pathbuf_filename(pathbuf: &path::Path) -> &str {
    pathbuf.file_name().unwrap().to_str().unwrap()
}
