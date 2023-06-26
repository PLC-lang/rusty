use futures::executor::block_on;
use sea_orm::{Database, ColumnTrait, EntityTrait, QueryFilter};


struct SqlReporter;

impl super::Reporter for SqlReporter {
    fn persist(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        block_on(self.persist_to_db(report))
    }
}

impl SqlReporter {
    async fn persist_to_db(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        //Connect to db
        let db_url = std::env::var("DATABASE_URL")?;
        let db = Database::connect(db_url).await?;
        
        //Push data
        Ok(())

    }
}
