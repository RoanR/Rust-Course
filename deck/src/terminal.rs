use std::cmp::max;

pub struct Terminal {
    size: u16,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            size: termsize::get().map_or_else(|| 40, |s| s.cols),
        }
    }

    pub fn clear(&self) {
        // clear screen using control character
        print!("{}[2J", 27 as char);
    }

    pub fn get_size(&mut self) {
        // Get the terminal size or use default of 40
        self.size = termsize::get().map_or_else(|| 40, |s| s.cols);
    }

    pub fn divider(&mut self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_size();
        for _ in 0..self.size {
            write!(f, "=")?;
        }
        write!(f, "\n")
    }

    pub fn centre_text(
        &mut self,
        f: &mut std::fmt::Formatter<'_>,
        text: String,
    ) -> std::fmt::Result {
        self.get_size();
        let centre = (Into::<usize>::into(self.size) - text.len()) / 2;
        let pad = (0..centre).map(|_| " ").collect::<String>();
        write!(f, "{}{}{}\n", pad, text, pad)
    }

    pub fn column_text(
        &mut self,
        f: &mut std::fmt::Formatter<'_>,
        texts: Vec<String>,
    ) -> std::fmt::Result {
        self.get_size();
        let col_size = Into::<usize>::into(self.size) / texts.len();

        let mut text_split: Vec<Vec<String>> = vec![];
        let mut max_lines = 0;

        for s in &texts {
            let col: Vec<String> = s.lines().map(str::to_string).collect();
            max_lines = max(max_lines, col.len());
            text_split.push(col);
        }

        for row in 0..max_lines {
            let mut this_col = "".to_string();
            for col in 0..texts.len() {
                let s = text_split[col]
                    .get(row)
                    .map_or_else(|| " ".to_string(), |s| s.to_string());
                let pad = (0..(col_size - s.chars().count()))
                    .map(|_| " ")
                    .collect::<String>();
                this_col.push_str(&s);
                this_col.push_str(&pad);
            }
            writeln!(f, "{}", this_col)?;
        }
        Ok(())
    }
}
