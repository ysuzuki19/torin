use super::*;

#[derive(Debug, PartialEq)]
pub enum Trigger {
    Feature(Feature),
    Date(Date),
}
