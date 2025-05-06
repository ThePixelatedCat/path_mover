use std::{
    error::Error,
    io::{BufReader, Seek},
};

use config::Config;
use path::Path;

pub mod config;
pub mod path;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for mut file in config.get_target_files()? {
        let mut path: Path = serde_json::from_reader(BufReader::new(&file))?;
        let (delta_x, delta_y) = rr_to_field(
            &config.amount,
            path.goal_end_state.rotation,
            config.sideways,
        );
        path = move_path(delta_x, delta_y, path)?;
        file.rewind()?;
        file.set_len(0)?;
        serde_json::to_writer_pretty(file, &path)?;
    }

    Ok(())
}

fn move_path(delta_x: f64, delta_y: f64, mut path: Path) -> Result<Path, Box<dyn Error>> {
    for waypoint in &mut path.waypoints {
        waypoint.anchor.x += delta_x;
        waypoint.anchor.y += delta_y;

        if let Some(point) = &mut waypoint.prev_control {
            point.x += delta_x;
            point.y += delta_y;
        }

        if let Some(point) = &mut waypoint.next_control {
            point.x += delta_x;
            point.y += delta_y;
        }
    }

    Ok(path)
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

    #[test]
    fn ninety_fore_pos() {
        assert_eq!(rr_to_field(&1.0, 90.0, false).0, 0.0);
        assert_eq!(rr_to_field(&1.0, 90.0, false).1, 1.0);
    }

    #[test]
    fn ninety_fore_neg() {
        assert_eq!(rr_to_field(&-1.0, 90.0, false).0, 0.0);
        assert_eq!(rr_to_field(&-1.0, 90.0, false).1, -1.0);
    }

    #[test]
    fn ninety_side_pos() {
        assert_eq!(rr_to_field(&1.0, 90.0, true).0, 1.0);
        assert_eq!(rr_to_field(&1.0, 90.0, true).1, 0.0);
    }

    #[test]
    fn ninety_side_neg() {
        assert_eq!(rr_to_field(&-1.0, 90.0, true).0, -1.0);
        assert_eq!(rr_to_field(&-1.0, 90.0, true).1, 0.0);
    }
}
