#[derive(Debug, Clone, PartialEq)]
pub struct Rule(String);

impl Rule {
    pub fn new<S>(rule: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(rule.as_ref().to_string())
    }

    pub fn name(&self) -> &String {
        &self.0
    }
}

impl<S> From<S> for Rule
where
    S: AsRef<str>,
{
    fn from(rule: S) -> Self {
        Rule::new(rule)
    }
}
