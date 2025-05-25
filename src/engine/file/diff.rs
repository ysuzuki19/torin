use crate::model::cutify::*;

pub struct Diff {
    begin: usize,
    end: usize,
    content: String,
}

impl Diff {
    pub fn lineno(&self) -> usize {
        self.begin + 1
    }

    fn unified_diff_format_header(&self) -> String {
        format!(
            "@@ -{},{} +{},0 @@",
            self.begin + 1,
            self.end - self.begin,
            self.begin + 1
        )
    }

    pub fn unified_diff_format(&self) -> String {
        format!(
            "{}\n{}",
            self.unified_diff_format_header().cutify().bold(),
            self.content.cutify().red()
        )
    }
}

pub struct DiffBuilder {
    begin: usize,
    lines: Vec<String>,
}

impl DiffBuilder {
    pub fn new(begin: usize) -> Self {
        Self {
            begin,
            lines: vec![],
        }
    }

    pub fn add(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn build(self) -> Diff {
        Diff {
            begin: self.begin,
            end: self.begin + self.lines.len(),
            content: self
                .lines
                .into_iter()
                .map(|s| format!("- {s}"))
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let mut builder = DiffBuilder::new(0);
        let first_line = "This is a test line.".to_string();
        builder.add(first_line);
        let second_line = "This is another line.".to_string();
        builder.add(second_line);

        assert_eq!(builder.begin, 0);
        assert_eq!(
            builder.lines,
            vec![
                "This is a test line.".to_string(),
                "This is another line.".to_string()
            ]
        );

        let diff = builder.build();
        assert_eq!(diff.begin, 0);
        assert_eq!(diff.end, 2);
        assert_eq!(
            diff.content,
            "- This is a test line.\n- This is another line."
        );

        let unified_diff = diff.unified_diff_format();
        assert_eq!(
            unified_diff,
            "\x1b[1;39m@@ -1,2 +1,0 @@\x1b[0m\n\x1b[31m- This is a test line.\n- This is another line.\x1b[0m"
        );
    }
}
