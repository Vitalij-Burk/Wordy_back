use translators::{GoogleTranslator, Translator};

pub async fn translate_text(
    source_text: &str,
    source_language: &str,
    target_language: &str,
) -> Result<String, translators::Error> {
    let google_translator = GoogleTranslator::default();

    let target_text = google_translator
        .translate_async(source_text, source_language, target_language)
        .await?;

    Ok(target_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_text() {
        assert_eq!(
            translate_text("Hello", "en", "de").await,
            Ok("Hallo".to_string())
        );
    }
}
