use crate::domain::traits::translator::translator::ITranslator;
use async_trait::async_trait;
use translators::{GoogleTranslator, Translator};

#[derive(Clone)]
pub struct TranslatorsTranslator;

#[async_trait]
impl ITranslator for TranslatorsTranslator {
    type Item = String;
    type Error = translators::Error;

    async fn translate_text(
        &self,
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
}
