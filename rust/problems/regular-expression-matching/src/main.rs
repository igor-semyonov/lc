fn main() {
    let _ = is_match(
        String::from("This is a string"),
        String::from("his.*is"),
    );
}

pub fn is_match(s: String, p: String) -> bool {
    let pattern: Pattern = p.into();
    pattern.check_match(&s)
}

impl Pattern {
    fn check_match(&self, s: &str) -> bool {
        let mut literal_runs: Vec<String> = vec![];
        let mut run = vec![];
        let mut last_run_added = false;
        for segment in &self.segments {
            if let PatternSegment::Literal(c) = segment {
                run.push(c.clone());
                last_run_added = false;
            } else {
                if run.len() > 0 {
                    literal_runs.push(
                        run.iter()
                            .collect(),
                    );
                    last_run_added = true;
                    run = vec![];
                }
            }
        }
        if !last_run_added {
            literal_runs.push(
                run.iter()
                    .collect(),
            );
        }
        let mut idxs = vec![];
        for (run_idx, run) in literal_runs
            .iter()
            .enumerate()
        {
            if run_idx == 0 {
                if let Some(idx) = s.find(run) {
                    idxs.push(idx);
                } else {
                    return false;
                }
            } else {
                let start_idx = idxs[run_idx - 1]
                    + literal_runs[run_idx - 1].len();
                if let Some(idx) = s[start_idx..].find(run)
                {
                    idxs.push(start_idx + idx);
                } else {
                    return false;
                }
            }
        }

        let mut matched_idxs = vec![false; s.len()];
        for (idx, run) in idxs
            .iter()
            .zip(literal_runs.iter())
        {
            for matched_idx in *idx..*idx + run.len() {
                matched_idxs[matched_idx] = true;
            }
        }

        let mut chars = s.chars();
        while let Some(c) = chars.next(){

        }

        matched_idxs
            .into_iter()
            .all(|idx| idx)
    }
}

#[derive(Debug)]
struct Pattern {
    segments: Vec<PatternSegment>,
}
#[allow(dead_code)]
#[derive(Debug)]
enum PatternSegment {
    Literal(char),
    Any,
    Repeated(PatternRepeatableItem),
}
#[allow(dead_code)]
#[derive(Debug)]
enum PatternRepeatableItem {
    Literal(char),
    Any,
}
#[derive(Debug)]
enum PatternItem {
    Literal(char),
    Dot,
    Star,
}
impl From<char> for PatternItem {
    fn from(value: char) -> Self {
        match value {
            '.' => PatternItem::Dot,
            '*' => PatternItem::Star,
            c => PatternItem::Literal(c),
        }
    }
}
impl From<String> for Pattern {
    fn from(value: String) -> Self {
        let mut segments = Vec::new();
        let mut items = value
            .chars()
            .map(|c| c.into())
            .collect::<Vec<PatternItem>>()
            .into_iter()
            .peekable();
        while let Some(item) = items.next() {
            if let Some(PatternItem::Star) = items.peek() {
                items.next();
                segments.push(
                        match item {
                            PatternItem::Dot=> {
                                PatternSegment::Repeated(PatternRepeatableItem::Any)
                            }
                            PatternItem::Literal(c) => {
                                PatternSegment::Repeated(PatternRepeatableItem::Literal(c))
                            }
                            PatternItem::Star => panic!("Two *'s repeated in pattern is not supported.")
                        },
                    );
            } else {
                segments.push(
                        match item {
                            PatternItem::Dot=> PatternSegment::Any,
                            PatternItem::Literal(c) => PatternSegment::Literal(c),
                            PatternItem::Star => panic!("Two *'s repeated in pattern is not supported.")
                        },
                    );
            }
        }
        Self {
            segments,
        }
    }
}
#[allow(dead_code)]
impl Pattern {
    fn len_min(&self) -> usize {
        self.segments
            .iter()
            .map(
                |segment| match segment {
                    PatternSegment::Literal(_)
                    | PatternSegment::Any => 1,
                    PatternSegment::Repeated(_) => 0,
                },
            )
            .sum()
    }
    fn len_max(&self) -> Option<usize> {
        self.segments
            .iter()
            .all(
                |segment| {
                    if let PatternSegment::Repeated(_) =
                        segment
                    {
                        false
                    } else {
                        true
                    }
                },
            )
            .then(|| self.len_min())
    }
}

#[cfg(test)]
mod tests;
