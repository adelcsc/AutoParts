mod test {
    use async_graphql::dataloader::Loader;
    use sea_orm::{Database, DatabaseConnection};
    use crate::SqliteLoader;

    #[test]
    fn test_altername_loader() {
        let loader = SqliteLoader {pool:Database::connect("sqlite:app.db")};
        let rs = loader.load(vec![crate::entities::alter_names::Model{id:4,..crate::entities::alter_names::Model::default()}].as_slice()).await.unwrap();
    }
}
