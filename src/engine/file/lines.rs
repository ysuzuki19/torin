#[derive(Debug, Clone, PartialEq)]
pub enum Flag {
    NotChange,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lines {
    data: Vec<(Flag, String)>,
}

impl Lines {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn lines(&self) -> Vec<String> {
        self.data
            .iter()
            .filter_map(|(flag, line)| {
                if matches!(flag, Flag::Delete) {
                    None
                } else {
                    Some(line.clone())
                }
            })
            .collect()
    }

    pub fn flagging(&mut self, flag: Flag, range: std::ops::Range<usize>) {
        let mut seek_index = 0;
        for e in self.data.iter_mut() {
            if matches!(e.0, Flag::Delete) {
                continue;
            }
            if range.start <= seek_index && seek_index < range.end {
                e.0 = flag.clone();
            }
            if range.end <= seek_index {
                break;
            }
            seek_index += 1;
        }
    }

    pub fn apply(&mut self) {
        self.data.retain(|(flag, _)| !matches!(flag, Flag::Delete));
    }

    pub fn join(&self) -> String {
        self.data
            .iter()
            .map(|(_, line)| line.to_owned())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<String> for Lines {
    fn from(s: String) -> Self {
        let data = s
            .split('\n')
            .map(|s| (Flag::NotChange, s.to_string()))
            .collect::<Vec<_>>();
        Self { data }
    }
}

#[cfg(test)]
impl From<Vec<String>> for Lines {
    fn from(data: Vec<String>) -> Self {
        let data = data
            .into_iter()
            .map(|s| (Flag::NotChange, s))
            .collect::<Vec<_>>();
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_from_string() {
        let lines = Lines::from("line1\nline2\nline3".to_string());
        assert_eq!(
            lines,
            Lines {
                data: vec![
                    (Flag::NotChange, "line1".to_string()),
                    (Flag::NotChange, "line2".to_string()),
                    (Flag::NotChange, "line3".to_string()),
                ],
            }
        );
    }

    #[test]
    fn test_lines_flagging() {
        let mut lines = Lines::from("line1\nline2\nline3\nline4\nline5".to_string());
        lines.flagging(Flag::Delete, 1..3);
        assert_eq!(
            lines,
            Lines {
                data: vec![
                    (Flag::NotChange, "line1".to_string()),
                    (Flag::Delete, "line2".to_string()),
                    (Flag::Delete, "line3".to_string()),
                    (Flag::NotChange, "line4".to_string()),
                    (Flag::NotChange, "line5".to_string()),
                ],
            }
        );
    }

    #[test]
    fn test_lines_flagging_on_flag_existed() {
        let mut lines = Lines::from("line1\nline2\nline3\nline4\nline5".to_string());
        lines.flagging(Flag::Delete, 0..2);
        lines.flagging(Flag::Delete, 1..2);
        assert_eq!(
            lines,
            Lines {
                data: vec![
                    (Flag::Delete, "line1".to_string()),
                    (Flag::Delete, "line2".to_string()),
                    (Flag::NotChange, "line3".to_string()),
                    (Flag::Delete, "line4".to_string()),
                    (Flag::NotChange, "line5".to_string()),
                ],
            }
        );
    }

    #[test]
    fn test_lines_apply() {
        let mut lines = Lines::from("line1\nline2\nline3\nline4\nline5".to_string());
        lines.flagging(Flag::Delete, 1..3);
        lines.apply();
        assert_eq!(
            lines,
            Lines {
                data: vec![
                    (Flag::NotChange, "line1".to_string()),
                    (Flag::NotChange, "line4".to_string()),
                    (Flag::NotChange, "line5".to_string()),
                ],
            }
        );
    }

    #[test]
    fn test_lines_join() {
        let mut lines = Lines::from("line1\nline2\nline3\nline4\nline5".to_string());
        lines.flagging(Flag::Delete, 1..3);
        lines.apply();
        assert_eq!(lines.join(), "line1\nline4\nline5");
    }
}
