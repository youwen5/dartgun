/// Utilities for translating Dartfiles into actual actions on the system.
use crate::dartfile::{Dartfile, Dotfile};
use std::os::unix::fs::symlink;

impl Dotfile {
    pub fn create_symlink(&self) -> Result<(), std::io::Error> {
        symlink(self.location.canonicalize()?, &self.destination)
    }
}

impl Dartfile {
    pub fn create_symlinks(&self) -> Result<(), std::io::Error> {
        for dot in self.dots.iter() {
            dot.create_symlink()?
        }
        Ok(())
    }
}
