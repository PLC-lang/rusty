use async_std::task::block_on;
use sqlx::mysql::MySqlPoolOptions;

use super::Host;

pub struct SqlReporter;

impl super::Reporter for SqlReporter {
    fn persist(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        block_on(self.persist_to_db(report))
    }
}

impl SqlReporter {
    async fn persist_to_db(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        // Connect to DB
        let db_url = std::env::var("DATABASE_URL")?;
        let db = MySqlPoolOptions::new().connect(&db_url).await?;

        // Get host ID
        let host = sqlx::query_as!(
            Host,
            "SELECT id, cpu, os, memory as mem from Host WHERE cpu = ? AND os = ? AND memory = ?",
            &report.host.cpu,
            &report.host.os,
            &report.host.mem,
        )
        .fetch_optional(&db)
        .await?;

        // Start transaction
        let trans = db.begin().await?;

        let id = match host {
            Some(Host { id, .. }) => id,
            _ => {
                // Commit new host and get its id
                sqlx::query!(
                    "INSERT INTO Host (cpu, os, memory) VALUES (?, ?, ?)",
                    &report.host.cpu,
                    &report.host.os,
                    &report.host.mem
                )
                .execute(&db)
                .await?;
                sqlx::query!("SELECT id from Host ORDER BY id DESC LIMIT 1").fetch_one(&db).await?.id
            }
        };

        // Commit the benchmark
        sqlx::query!(
            "INSERT INTO Report (host_id, timestamp, commit) VALUES (?, ?, ?)",
            id,
            &report.timestamp,
            &report.commit
        )
        .execute(&db)
        .await?;

        let id = sqlx::query!("SELECT id from Report ORDER BY id DESC LIMIT 1").fetch_one(&db).await?.id;
        for (name, time) in report.metrics {
            sqlx::query!("INSERT INTO Metric (report_id, name, time) VALUES (?, ?, ?)", id, &name, &time)
                .execute(&db)
                .await?;
        }

        // Push data
        trans.commit().await?;
        db.close().await;

        Ok(())
    }
}
