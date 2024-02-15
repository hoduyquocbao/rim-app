// Các thư viện chuẩn của Rust
use std::fmt;

// Định nghĩa kiểu dữ liệu cho các thành phần ngôn ngữ
#[derive(Debug)]
struct Character(String);
#[derive(Debug)]
struct Accent(String);
#[derive(Debug)]
struct Vowel(String);
#[derive(Debug)]
struct Consonant(String);
#[derive(Debug)]
struct Syllable {
    consonant: Option<Consonant>,
    vowel: Vowel,
    consonant_ending: Option<Consonant>,
}
#[derive(Debug)]
struct Word(Vec<Syllable>);
#[derive(Debug)]
struct Phrase(Vec<Word>);
#[derive(Debug)]
enum Punctuation {
    Dot,
    Comma,
    Exclamation,
    Question,
}
#[derive(Debug)]
struct Sentence(Phrase, Punctuation);
#[derive(Debug)]
struct CompoundSentence(Sentence, String, Sentence); // String: conjunction
#[derive(Debug)]
struct ComplexSentence(Clause, String, Sentence); // String: conjunction
#[derive(Debug)]
struct ConditionalSentence(Clause, Sentence, Option<Sentence>); // Option<Sentence>: else_clause
#[derive(Debug)]
struct RelativeClause(NounClause);
#[derive(Debug)]
struct PassiveVoice(String); // String: noun
#[derive(Debug)]
struct ActiveVoice(String); // String: noun
#[derive(Debug)]
struct DirectSpeech(Sentence);
#[derive(Debug)]
struct IndirectSpeech(Sentence);
#[derive(Debug)]
struct ImperativeSentence(Word);
#[derive(Debug)]
struct Question(Vec<String>, Vec<Sentence>); // Vec<String>: question_words, Vec<Sentence>: sentences
#[derive(Debug)]
struct Exclamation(Sentence);
#[derive(Debug)]
enum SentenceType {
    Interrogative(Question),
    Exclamatory(Exclamation),
    Declarative(Sentence),
}
#[derive(Debug)]
struct Text(Vec<SentenceType>);

// Định nghĩa struct Clause và NounClause
#[derive(Debug)]
struct Clause(String);
#[derive(Debug)]
struct NounClause(String);

// Implement Display cho các struct
impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for NounClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Code của Display cho các struct khác không thay đổi
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Accent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Vowel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Consonant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.consonant {
            Some(c) => write!(f, "{}", c)?,
            None => {}
        }
        write!(f, "{}", self.vowel)?;
        match &self.consonant_ending {
            Some(c) => write!(f, "{}", c)?,
            None => {}
        }
        Ok(())
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for syllable in &self.0 {
            write!(f, "{}", syllable)?;
        }
        Ok(())
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, word) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", word)?;
        }
        Ok(())
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Punctuation::Dot => write!(f, "."),
            Punctuation::Comma => write!(f, ","),
            Punctuation::Exclamation => write!(f, "!"),
            Punctuation::Question => write!(f, "?"),
        }
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for CompoundSentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl fmt::Display for ComplexSentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl fmt::Display for ConditionalSentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nếu {} thì {}", self.0, self.1)?;
        match &self.2 {
            Some(else_clause) => write!(f, " ngược lại {}", else_clause),
            None => Ok({})
        };
        Ok(())
    }
}

impl fmt::Display for RelativeClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for PassiveVoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "được {} bởi {}", self.0, self.0)
    }
}

impl fmt::Display for ActiveVoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bởi {}", self.0, self.0)
    }
}

impl fmt::Display for DirectSpeech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nói rằng {}", self.0)
    }
}

impl fmt::Display for IndirectSpeech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ImperativeSentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.0.join(" "))?;
        for sentence in &self.1 {
            write!(f, "{}", sentence)?;
        }
        Ok(())
    }
}

impl fmt::Display for Exclamation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for SentenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SentenceType::Interrogative(question) => write!(f, "{}", question),
            SentenceType::Exclamatory(exclamation) => write!(f, "{}", exclamation),
            SentenceType::Declarative(sentence) => write!(f, "{}", sentence),
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for sentence_type in &self.0 {
            write!(f, "{} ", sentence_type)?;
        }
        Ok(())
    }
}

fn main() {
    // Tạo và in ra một ví dụ Text
    let example_text = Text(vec![
        SentenceType::Declarative(Sentence(
            Phrase(vec![
                Word(vec![Syllable {
                    consonant: Some(Consonant("H".to_string())),
                    vowel: Vowel("ôm".to_string()),
                    consonant_ending: None,
                }]),
                Word(vec![Syllable {
                    consonant: None,
                    vowel: Vowel("nay".to_string()),
                    consonant_ending: None,
                }]),
            ]),
            Punctuation::Comma,
        )),
        SentenceType::Declarative(Sentence(
            Phrase(vec![
                Word(vec![Syllable {
                    consonant: None,
                    vowel: Vowel("trời".to_string()),
                    consonant_ending: None,
                }]),
                Word(vec![Syllable {
                    consonant: None,
                    vowel: Vowel("đẹp".to_string()),
                    consonant_ending: None,
                }]),
            ]),
            Punctuation::Dot,
        )),
        SentenceType::Interrogative(Question(
            vec!["Tại sao".to_string()],
            vec![Sentence(
                Phrase(vec![
                    Word(vec![Syllable {
                        consonant: None,
                        vowel: Vowel("bạn".to_string()),
                        consonant_ending: None,
                    }]),
                    Word(vec![Syllable {
                        consonant: Some(Consonant("k".to_string())),
                        vowel: Vowel("hông".to_string()),
                        consonant_ending: None,
                    }]),
                ]),
                Punctuation::Comma,
            )],
        )),
    ]);

    println!("{}", example_text);
}
