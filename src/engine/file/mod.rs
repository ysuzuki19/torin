mod diff;
mod lines;

use lines::Lines;

use crate::prelude::*;

pub enum Destination {
    Overwrite,
    #[allow(unused)]
    File(String),
    #[cfg(test)]
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    path: Option<String>,
    lines: Lines,
}

impl File {
    pub fn load<S>(path: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let content = std::fs::read_to_string(path.as_ref())?;
        Ok(Self {
            path: Some(path.as_ref().to_string()),
            lines: Lines::from(content),
        })
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.lines()
    }

    pub fn flagging(&mut self, begin: usize, end: usize) {
        let end = if end >= self.lines.len() {
            self.lines.len()
        } else {
            end + 1
        };
        self.lines.flagging(lines::Flag::Delete, begin..end);
    }

    pub fn apply(&mut self) {
        self.lines.apply();
    }

    pub fn diffs(&self) -> Vec<diff::Diff> {
        self.lines.diffs()
    }

    pub fn dump(&self, dest: Destination) -> Result<String> {
        let contents = self.lines.join();
        match (&self.path, dest) {
            (Some(path), Destination::Overwrite) => {
                std::fs::write(path, &contents)?;
            }
            (None, Destination::Overwrite) => trace!(
                "Cannot overwrite without a file path. Use `File::load` to load a file first."
            )?,

            (_, Destination::File(path)) => std::fs::write(path, &contents)?,
            #[cfg(test)]
            (_, Destination::Noop) => {}
        }
        Ok(contents)
    }
}

impl File {
    #[cfg(test)]
    pub fn mock(lines: Vec<String>) -> Self {
        Self {
            path: None,
            lines: Lines::from(lines),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn relative(relative: &str) -> Result<String> {
        let source_file = std::path::Path::new(file!());
        let dir = source_file.parent().unwrap();
        Ok(dir
            .join(relative)
            .to_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid path"))?
            .to_string())
    }

    #[test]
    fn test_load() {
        testing::with_trace(|| {
            let cases = vec![
                "testdata/sample_0.txt",
                "testdata/sample_1.txt",
                "testdata/sample_2.txt",
            ];
            for case in cases {
                let path = relative(case)?;
                let real = std::fs::read_to_string(&path)?;
                let f = File::load(path)?;
                let loaded = f.lines.join();
                assert_eq!(loaded, real);
            }
            Ok(())
        });
    }

    #[test]
    fn test_dump_not_changed() {
        testing::with_trace(|| {
            let path = relative("testdata/sample_0.txt")?;
            let before = std::fs::read_to_string(&path)?;
            let after = File::load(path.clone())?.dump(super::Destination::Noop)?;
            assert_eq!(before, after);
            Ok(())
        })
    }

    #[test]
    fn test_drain() {
        let f = File::mock(vec![
            "line1".to_string(), // index=0
            "line2".to_string(), // index=1
            "line3".to_string(), // index=2
            "line4".to_string(), // index=3
            "line5".to_string(), // index=4
            "line6".to_string(), // index=5
        ]);
        struct Case {
            name: &'static str,
            start: usize,
            end: usize,
            expected: Vec<&'static str>,
        }
        for case in [
            Case {
                name: "remove all",
                start: 0,
                end: 5,
                expected: vec![],
            },
            Case {
                name: "remove subslice from beginning",
                start: 0,
                end: 2,
                expected: vec!["line4", "line5", "line6"],
            },
            Case {
                name: "remove subslice",
                start: 2,
                end: 4,
                expected: vec!["line1", "line2", "line6"],
            },
            Case {
                name: "remove subslice from end",
                start: 4,
                end: 6,
                expected: vec!["line1", "line2", "line3", "line4"],
            },
        ] {
            let mut f = f.clone();
            f.flagging(case.start, case.end);
            f.apply();
            assert_eq!(f.lines(), case.expected, "{}", case.name);
        }
    }
}
