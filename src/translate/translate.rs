use crate::models::word_pair::WordPair;
use translators::{GoogleTranslator, Translator};

pub async fn translate_text(
    user_id: &i32,
    source_text: &str,
    source_language: &str,
    target_language: &str,
) -> Result<WordPair, Box<dyn std::error::Error>> {
    let google_translator = GoogleTranslator::default();

    let target_text = google_translator
        .translate_async(source_text, source_language, target_language)
        .await?;

    let word_pair = WordPair::new(
        user_id,
        &target_text,
        source_text,
        target_language,
        source_language,
    );

    Ok(word_pair)
}
