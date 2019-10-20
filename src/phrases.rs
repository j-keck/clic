use xstd::prelude::*;

#[derive(Debug)]
pub struct Phrases {
    phrases: Vec<String>,
}

impl Phrases {
    pub fn parse(str: &str) -> Self {
        let mut phrases = Vec::<String>::new();
        let mut buffer = String::new();
        let mut quotation_mark: Option<char> = None;

        // some helper
        fn is_quotation_mark(c: char) -> (bool, char) {
            (c == '\'' || c == '"', c)
        }
        fn is_space(c: char) -> bool {
            c.is_ascii_whitespace()
        }
        fn is_masked_quotation(iter: &MementoIter<std::str::Chars>) -> bool {
            iter.prev() == Some(&'\\')
                && iter.cur().map(|c| is_quotation_mark(*c).0).unwrap_or(false)
        }

        // parse the given str
        let mut cursor = str.chars().memento();
        loop {
            if let Some(c) = cursor.next() {
                if is_masked_quotation(&cursor) {
                    buffer.push(c);
                } else if let (true, mark) = is_quotation_mark(c) {
                    if quotation_mark == Some(mark) {
                        quotation_mark = None;
                    } else if quotation_mark.is_none() {
                        quotation_mark = Some(mark);
                    } else {
                        buffer.push(c);
                    }
                } else if is_space(c) && quotation_mark.is_none() {
                    phrases.push(buffer);
                    buffer = String::new();
                } else {
                    buffer.push(c);
                }
            } else {
                if !buffer.is_empty() {
                    phrases.push(buffer);
                }
                break;
            }
        }

        Phrases { phrases }
    }

    pub fn vec(&self) -> &Vec<String> {
        &self.phrases
    }
}

impl IntoIterator for Phrases {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.phrases.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words() {
        let expected = vec!["this", "are", "some", "words"];
        let phrases = Phrases::parse("this are some words");
        assert_eq!(&expected, phrases.vec());
    }

    #[test]
    fn quoted_text() {
        let expected = vec!["bash", "-c", "echo hello"];
        let phrases = Phrases::parse("bash -c 'echo hello'");
        assert_eq!(&expected, phrases.vec());
    }

    #[test]
    fn masked_quotation() {
        let expected = vec!["bash", "-c", r#"echo 'I\'m hungry'"#];
        let phrases = Phrases::parse(r#"bash -c "echo 'I\'m hungry'"#);
        assert_eq!(&expected, phrases.vec());
    }

}
