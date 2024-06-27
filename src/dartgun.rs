/// Utilities for translating Dartfiles into actual actions on the system.
use crate::dartfile::{Dartfile, Dotfile};
use std::{collections::HashSet, os::unix::fs::symlink};

fn have_common_elements(vec1: &[String], vec2: &[String]) -> bool {
    let set: HashSet<_> = vec1.iter().collect();
    vec2.iter().any(|item| set.contains(item))
}

impl Dotfile {
    pub fn create_symlink(&self, machine_identifiers: &[String]) -> Result<(), std::io::Error> {
        if have_common_elements(&self.identifiers, machine_identifiers) {
            return symlink(self.location.canonicalize()?, &self.destination);
        }
        Ok(())
    }
}

impl Dartfile {
    pub fn create_symlinks(&self) -> Result<(), std::io::Error> {
        for dot in self.dots.iter() {
            dot.create_symlink(&self.machine.identifiers)?
        }
        Ok(())
    }
}
