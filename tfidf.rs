use std::collections::{HashMap, HashSet};

struct Tokenizer;

impl Tokenizer {
    fn tokenize(text: &str) -> HashSet<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Document {
    content: String,
    tfidf_scores: HashMap<String, f64>,
}

impl Document {
    fn new(content: &str) -> Self {
        Document {
            content: content.to_string(),
            tfidf_scores: HashMap::new(),
        }
    }

    fn compute_tfidf(&mut self, idf_scores: &HashMap<String, f64>) {
        let tokens = Tokenizer::tokenize(&self.content);
        let mut term_frequency: HashMap<String, f64> = HashMap::new();

        for token in &tokens {
            *term_frequency.entry(token.clone()).or_insert(0.0) += 1.0;
        }

        let num_unique_tokens = tokens.len() as f64;

        for (term, frequency) in term_frequency {
            if let Some(idf_score) = idf_scores.get(&term) {
                let tfidf = (frequency / num_unique_tokens) * idf_score;
                self.tfidf_scores.insert(term, tfidf);
            }
        }
    }

    fn normalize_tfidf_scores(&mut self) {
        let norm: f64 = self
            .tfidf_scores
            .values()
            .map(|v| v.powi(2))
            .sum::<f64>()
            .sqrt();
        for value in self.tfidf_scores.values_mut() {
            *value /= norm;
        }
    }

    fn get_top_keywords(&self, k: usize) -> Vec<String> {
        let mut keywords: Vec<_> = self.tfidf_scores.iter().collect();
        keywords.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        keywords
            .into_iter()
            .take(k)
            .map(|(term, _)| term.clone())
            .collect()
    }
}

struct TFIDF {
    idf_scores: HashMap<String, f64>,
}

impl TFIDF {
    fn new() -> Self {
        TFIDF {
            idf_scores: HashMap::new(),
        }
    }

    fn transform(&self, document: &str) -> Document {
        let mut doc = Document::new(document);
        doc.compute_tfidf(&self.idf_scores);
        doc
    }

    fn transform_all(&self, documents: &[&str]) -> Vec<Document> {
        documents.iter().map(|doc| self.transform(doc)).collect()
    }

    fn search_by_keyword<'a>(&'a self, keyword: &str, documents: &'a [Document]) -> Vec<&Document> {
        documents
            .iter()
            .filter(|doc| doc.tfidf_scores.contains_key(keyword))
            .collect()
    }
}

impl TFIDF {
    // ... (các phần khác của impl không thay đổi)

    fn fit(&mut self, documents: &[&str]) {
        let num_documents = documents.len() as f64;

        let mut document_term_frequency: Vec<HashMap<String, f64>> = Vec::new();

        for document in documents {
            let tokens = Tokenizer::tokenize(document);
            let mut term_frequency: HashMap<String, f64> = HashMap::new();

            for token in &tokens {
                *term_frequency.entry(token.clone()).or_insert(0.0) += 1.0;
            }

            document_term_frequency.push(term_frequency);
        }

        for term_frequency in &document_term_frequency {
            for (term, _) in term_frequency {
                *self.idf_scores.entry(term.clone()).or_insert(0.0) += 1.0;
            }
        }

        for (term, score) in &mut self.idf_scores {
            *score = ((num_documents / (*score + 1.0)) + 1.0).ln(); // Sửa đổi ở đây
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "The quick brown fox.";
        let expected_tokens: HashSet<String> = ["the", "quick", "brown", "fox"].iter().map(|s| s.to_string()).collect();
        assert_eq!(Tokenizer::tokenize(text), expected_tokens);
    }

    #[test]
    fn test_compute_tfidf() {
        let mut tfidf = TFIDF::new();
        let documents = vec![
            "Blue house.",
            "Red house.",
            "Blue window."
        ];
        tfidf.fit(&documents);

        let mut doc = Document::new("Blue house.");
        doc.compute_tfidf(&tfidf.idf_scores);

        let blue_score = doc.tfidf_scores.get("blue").unwrap();
        let house_score = doc.tfidf_scores.get("house").unwrap();

        let expected_blue_score = (1.0 / 2.0) * (f64::ln(3.0 / 2.0) + 1.0);
        let expected_house_score = (1.0 / 2.0) * (f64::ln(3.0 / 2.0) + 1.0);

        assert!((blue_score - expected_blue_score).abs() < f64::EPSILON);
        assert!((house_score - expected_house_score).abs() < f64::EPSILON);
    }

    // Thêm các test khác tương tự...
}

fn main() {
    let documents = vec![
        "This is the first document.",
        "This document is the second document.",
        "And this is the third one.",
        "Is this the first document?",
    ];

    let mut tfidf = TFIDF::new();
    tfidf.fit(&documents);

    let mut tfidf_documents = tfidf.transform_all(&documents);

    for doc in &mut tfidf_documents {
        doc.normalize_tfidf_scores(); // Chuẩn hóa điểm số TF-IDF
        let top_keywords = doc.get_top_keywords(3);
        println!("Top keywords: {:?}", top_keywords);
    }

    let keyword_to_search = "document";
    let search_results = tfidf.search_by_keyword(keyword_to_search, &tfidf_documents);
    println!("Search results for '{}':", keyword_to_search);
    for (i, result) in search_results.iter().enumerate() {
        println!("{}: {:?}", i + 1, result);
    }
}
