use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;

/// The dartfile module handles parsing the `dotfile.toml` into
/// a well-typed and self-validating data structure.

/// The raw form of a dotfile entry. Paths are stored as strings.
///
/// Should not be used outside of module internals. Its primary purpose
/// is to provide an intermediary between the raw parsed TOML and
/// the strongly typed Dotfile struct
#[derive(Debug)]
struct DotfileRaw {
    location: String,
    destination: String,
    strategy: String,
    identifiers: Vec<String>,
}

impl DotfileRaw {
    fn determine_strategy(&self) -> Result<Strategy, String> {
        match self.strategy.as_str() {
            "hardlink" => Ok(Strategy::Hardlink),
            "symlink" => Ok(Strategy::Symlink),
            _ => Err(String::from(
                "Strategy must be either 'symlink' or 'hardlink'",
            )),
        }
    }

    fn to_dotfile(&self) -> Result<Dotfile, String> {
        let location_path = PathBuf::from(&self.location);
        let destination_path = PathBuf::from(&self.destination);

        Ok(Dotfile {
            location: location_path,
            destination: destination_path,
            strategy: self.determine_strategy()?,
            identifiers: self.identifiers.clone(),
        })
    }
    // TODO: improve error handling for parsing from raw TOML

    /// Create a new `DotfileRaw` from a `toml::Table`, which is a `Vec<Value>`.
    /// Will panic if the table does not contain the fields specified.
    fn from_table(table: Table) -> DotfileRaw {
        let location = table
            .get("location")
            .ok_or("Missing 'location' field.")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let destination = table
            .get("destination")
            .ok_or("Missing 'destination' field.")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let strategy = table
            .get("strategy")
            .ok_or("Missing 'strategy' field.")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let identifiers = table
            .get("identifiers")
            .ok_or("Missing 'identifiers' field.")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();

        DotfileRaw {
            location,
            destination,
            strategy,
            identifiers,
        }
    }
}

/// The configuration object parsed from the `config` field in `dartgun.toml`
#[derive(Debug)]
pub struct Machine {
    pub identifiers: Vec<String>,
}

impl Machine {
    // TODO: improve error handling for parsing config

    /// Generate a `Config` from a raw `dartfile.toml`.
    /// Will panic! if the format is invalid.
    fn from_table(table: Table) -> Machine {
        let identifiers = table
            .get("identifiers")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        Machine { identifiers }
    }
}

/// A strongly typed Dartfile (aka `dartgun.toml`). Users of this
/// struct can assume that the dartfile is at least semantically valid
#[derive(Debug)]
pub struct Dartfile {
    pub machine: Machine,
    pub dots: Vec<Dotfile>,
}

impl Dartfile {
    /// Validates the Dartfile by checking each Dotfile entry to ensure
    /// the paths specified by `location` are accessible.
    pub fn validate(&self) -> Result<(), String> {
        for dotfile in self.dots.iter() {
            if dotfile.validate().is_err() {
                return Err("Invalid dotfile.".to_string());
            }
        }
        Ok(())
    }
}

/// Represents which strategy to use when deploying dotfiles across system.
#[derive(Debug)]
pub enum Strategy {
    Hardlink,
    Symlink,
}

/// A strongly-typed Dotfile entry
#[derive(Debug)]
pub struct Dotfile {
    pub location: PathBuf,
    pub destination: PathBuf,
    pub strategy: Strategy,
    pub identifiers: Vec<String>,
}

impl Dotfile {
    /// Validates the entry by checking whether the `location` paths
    /// specified are accessible.
    pub fn validate(&self) -> Result<(), String> {
        let path_exists = self.location.try_exists();
        match path_exists {
            Ok(true) => Ok(()),
            Ok(false) => Err("Could not follow broken symlink.".to_string()),
            Err(_) => Err("An error occurred. Does the path exist?".to_string()),
        }
    }
}

/// Takes a path to a `dartgun.toml` and produces a well-typed Dartfile object.
/// Currently crashes on any parse errors, but this behavior will likely change in the future.
pub fn parse(dartgun_path: &Path, machine_path: &Path) -> Dartfile {
    let dartgun_toml = fs::read_to_string(dartgun_path)
        .expect("Couldn't read dartgun.toml")
        .parse::<Table>()
        .expect("Couldn't parse the TOML.");
    let machine_toml = fs::read_to_string(machine_path)
        .expect("Couldn't read machine.toml")
        .parse::<Table>()
        .expect("Couldn't parse the TOML.");

    let dots_raw = dartgun_toml.get("dots").unwrap().as_array().unwrap();

    let machine = Machine::from_table(machine_toml);
    let dots = dots_raw
        .iter()
        .map(|x| {
    match DotfileRaw::from_table(x.as_table().unwrap().clone()).to_dotfile() {
                Ok(dotfile) => dotfile,
                Err(_) => panic!("An error has occurred parsing the `dartgun.toml` file. Please make sure it is in the correct format.")
            }
        })
        .collect::<Vec<Dotfile>>();

    Dartfile { machine, dots }
}
