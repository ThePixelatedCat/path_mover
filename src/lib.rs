use std::{error, fs, path};

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    for file_path in config.get_target_files()? {
        let (delta_x, delta_y) =
            rr_to_field(&config.amount, get_path_angle(&file_path)?, config.sideways);
        move_path(delta_x, delta_y, &file_path)?;
    }

    Ok(())
}

fn move_path(
    delta_x: f64,
    delta_y: f64,
    file_path: &path::PathBuf,
) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(file_path)?;

    let mut lines: Vec<String> = contents.lines().map(|line| line.to_owned()).collect();

    for line in &mut lines {
        if line.contains("\"y\": ") {
            let current_y = line
                .trim()
                .strip_prefix("\"y\": ")
                .ok_or("can't find y in y line")?
                .parse::<f64>()?;

            *line = format!("        \"y\": {}", current_y + delta_y);
        } else if line.contains("\"x\": ") {
            let current_x = line
                .trim()
                .strip_prefix("\"x\": ")
                .ok_or("can't find x in x line")?
                .strip_suffix(",")
                .ok_or("x line doesn't end in comma")?
                .parse::<f64>()?;

            *line = format!("        \"x\": {},", current_x + delta_x);
        };

        (*line).push('\n');
    }

    fs::write(file_path, lines.concat())?;

    Ok(())
}

fn get_path_angle(file_path: &path::PathBuf) -> Result<f64, Box<dyn error::Error>> {
    let contents = fs::read_to_string(file_path)?;

    let path_angle = contents
        .lines()
        .find(|line| line.contains("\"rotation\":"))
        .ok_or("can't find rotation in path")?
        .trim()
        .strip_prefix("\"rotation\": ")
        .ok_or("can't find rotation in rotation line")?
        .parse()?;

    Ok(path_angle)
}

fn rr_to_field(distance: &f64, robot_angle: f64, sideways: bool) -> (f64, f64) {
    let robot_angle = if sideways {
        robot_angle + 90.0
    } else {
        robot_angle
    };

    let opp = distance * robot_angle.to_radians().sin();
    let adj = distance * robot_angle.to_radians().cos();

    (adj, opp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_parsing() {
        assert_eq!(get_path_angle(&path::PathBuf::from("C:\\Users\\acfro\\Github\\2025Reefscape\\src\\main\\deploy\\pathplanner\\paths\\ra.path")).unwrap(), 90.0);
    }

    #[test]
    fn zero_fore_pos() {
        assert_eq!(rr_to_field(&1.0, 0.0, false).0, 1.0);
        assert_eq!(rr_to_field(&1.0, 0.0, false).1, 0.0);
    }

    #[test]
    fn zero_fore_neg() {
        assert_eq!(rr_to_field(&-1.0, 0.0, false).0, -1.0);
        assert_eq!(rr_to_field(&-1.0, 0.0, false).1, 0.0);
    }

    #[test]
    fn zero_side_pos() {
        assert_eq!(rr_to_field(&1.0, 0.0, true).0, 0.0);
        assert_eq!(rr_to_field(&1.0, 0.0, true).1, 1.0);
    }

    #[test]
    fn zero_side_neg() {
        assert_eq!(rr_to_field(&-1.0, 0.0, true).0, 0.0);
        assert_eq!(rr_to_field(&-1.0, 0.0, true).1, -1.0);
    }

    //#[test]
    fn ninety_fore_pos() {
        assert_eq!(rr_to_field(&1.0, 90.0, false).0, 0.0);
        assert_eq!(rr_to_field(&1.0, 90.0, false).1, 1.0);
    }

    //#[test]
    fn ninety_fore_neg() {
        assert_eq!(rr_to_field(&-1.0, 90.0, false).0, 0.0);
        assert_eq!(rr_to_field(&-1.0, 90.0, false).1, -1.0);
    }

    //#[test]
    fn ninety_side_pos() {
        assert_eq!(rr_to_field(&1.0, 90.0, true).0, 1.0);
        assert_eq!(rr_to_field(&1.0, 90.0, true).1, 0.0);
    }

    //#[test]
    fn ninety_side_neg() {
        assert_eq!(rr_to_field(&-1.0, 90.0, true).0, -1.0);
        assert_eq!(rr_to_field(&-1.0, 90.0, true).1, 0.0);
    }
}
