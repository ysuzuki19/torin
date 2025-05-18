#[derive(Debug, PartialEq)]
pub struct Feature(String);

impl Feature {
    pub fn new<S>(feature: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(feature.as_ref().to_string())
    }

    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl<S> From<S> for Feature
where
    S: AsRef<str>,
{
    fn from(feature: S) -> Self {
        Feature::new(feature)
    }
}
