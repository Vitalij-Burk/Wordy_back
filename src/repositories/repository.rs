pub trait Repository: Send + Sync {
    type Item;
    type Error;

    async fn insert(&self, item: &Self::Item) -> Result<Self::Item, Self::Error>;
}
