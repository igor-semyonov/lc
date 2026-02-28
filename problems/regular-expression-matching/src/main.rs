fn main() {
    let _ = is_match(
        String::from("This is a string"),
        String::from("a s."),
    );
}

fn is_match(s: String, p: String) -> bool {
    #[derive(Debug)]
    struct Pattern {
        segments: Vec<PatternSegment>,
    }
    #[derive(Debug)]
    enum PatternSegment {
        Literal(char),
        Any,
        Repeated(PatternRepeatableItem),
    }
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
                if let Some(PatternItem::Star) =
                    items.peek()
                {
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

    let pattern: Pattern = p.into();
    println!("{pattern:?}");
    println!("{}", pattern.len_min());
    println!("{:?}", pattern.len_max());
    false
}

#[cfg(test)]
mod tests;
