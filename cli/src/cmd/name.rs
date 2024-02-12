use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

pub trait CommandNameConstraints: Eq + PartialEq + Hash + Clone + Display + FromStr {}
impl<T> CommandNameConstraints for T where T: Eq + PartialEq + Hash + Clone + Display + FromStr {}
