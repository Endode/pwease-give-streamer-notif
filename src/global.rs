pub struct Version {
    major: i8,
    minor: i8,
    revision: i8,
}

impl Version {
    pub fn get_string(&self) -> String {
        return "".to_owned()
            + &self.major.to_string()
            + "."
            + &self.minor.to_string()
            + "."
            + &self.revision.to_string();
    }
}

pub const VERSION: Version = Version {
    major: 1,
    minor: 0,
    revision: 0,
};

#[cfg(not(debug_assertions))]
pub const PROGRAM_NAME: &str = "Pwease Give Streamer Notif";
#[cfg(debug_assertions)]
pub const PROGRAM_NAME: &str = "Pwease Give Streamer Notif | Debug";
