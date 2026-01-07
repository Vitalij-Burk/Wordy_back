use thiserror::Error;

use crate::{
    models::word_pair::{CreateWordPair, WordPair},
    repositories::{repository::Repository, word_pair_repository::IWordPairRepository},
    translate::translate::translate_text,
};

#[derive(Clone)]
pub struct WordPairService<T> {
    repo: T,
}

#[derive(Debug, Error)]
pub enum WordPairServiceError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Google Translate API error")]
    GoogleTranslateAPI(#[from] translators::Error),
}

impl<T> WordPairService<T>
where
    T: Repository<Item = WordPair, Error = sqlx::Error>,
{
    pub fn new(repo: T) -> Self {
        Self { repo: repo }
    }

    pub async fn create(
        &self,
        user_id: &i32,
        params: &CreateWordPair,
    ) -> Result<WordPair, WordPairServiceError> {
        let target_text = translate_text(
            &params.source_text,
            &params.source_language,
            &params.target_language,
        )
        .await?;

        let word_pair = WordPair::new(
            &user_id,
            &target_text,
            &params.source_text,
            &params.target_language,
            &params.source_language,
        );

        let res = self.repo.insert(&word_pair).await?;

        Ok(res)
    }
}

impl<T> WordPairService<T>
where
    T: IWordPairRepository,
{
    pub async fn get_by_user_id(
        &self,
        user_id: &i32,
    ) -> Result<Vec<WordPair>, WordPairServiceError> {
        let res = self.repo.select_by_user_id(&user_id).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    struct TestWordPairRepository {
        _db: i32,
    }

    #[async_trait]
    impl Repository for TestWordPairRepository {
        type Item = WordPair;
        type Error = sqlx::Error;

        async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item.clone())
        }
    }

    #[async_trait]
    impl IWordPairRepository for TestWordPairRepository {
        async fn select_by_user_id(&self, user_id: &i32) -> Result<Vec<Self::Item>, Self::Error> {
            Ok(vec![WordPair {
                id: 1234,
                user_id: *user_id,
                target_text: "Hallo".to_string(),
                source_text: "Hello".to_string(),
                target_language: "de".to_string(),
                source_language: "en".to_string(),
            }])
        }
    }

    #[tokio::test]
    async fn test_create() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_params = CreateWordPair {
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
        };
        let test_user_id = 1234567;

        let res = word_pair_service
            .create(&test_user_id, &test_params)
            .await
            .unwrap();

        assert_eq!(res.user_id, test_user_id);
    }

    #[tokio::test]
    async fn test_get_by_user_id() {
        let repo = TestWordPairRepository { _db: 12345 };

        let word_pair_service = WordPairService::new(repo);

        let test_user_id = 1234567;

        let res = word_pair_service
            .get_by_user_id(&test_user_id)
            .await
            .unwrap();

        let val = vec![WordPair {
            id: 1234,
            user_id: test_user_id,
            target_text: "Hallo".to_string(),
            source_text: "Hello".to_string(),
            target_language: "de".to_string(),
            source_language: "en".to_string(),
        }];

        assert_eq!(res[0].user_id, val[0].user_id);
        assert_eq!(res[0].id, val[0].id);
    }
}
