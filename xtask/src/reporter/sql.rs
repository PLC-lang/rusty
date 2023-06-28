use async_std::task::block_on;
use sqlx::{mysql::MySqlPoolOptions, migrate::Migrate};

use super::Host;

pub struct SqlReporter;

impl super::Reporter for SqlReporter {
    fn persist(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        println!("Before async");
        let res = block_on(self.persist_to_db(report));
        println!("After async");
        res
    }
}

impl SqlReporter {
    async fn persist_to_db(&self, report: super::BenchmarkReport) -> anyhow::Result<()> {
        println!("Persisting to db");
        //Connect to db
        let db_url = dbg!(std::env::var("DATABASE_URL"))?;
        let db = MySqlPoolOptions::new().connect(&db_url).await?;

        // get host id
        let host = sqlx::query_as!(
            Host,
            "SELECT id, cpu, os, memory as mem from Host WHERE cpu = ? AND os = ? AND memory = ?",
            &report.host.cpu,
            &report.host.os,
            &report.host.mem,
        ).fetch_optional(&db).await?;

        //Start transaction
        let trans = db.begin().await?;
        let id = match host {
            Some(Host{id, .. }) => id,
            _ => {
                //Commit new host and get its id
                sqlx::query!("INSERT INTO Host (cpu, os, memory) VALUES (?, ?, ?)", 
            &report.host.cpu,
            &report.host.os,
            &report.host.mem).execute(&db).await?;
                sqlx::query!("SELECT LAST_INSERT_ID() as id").fetch_one(&db).await?.id
            }
        };

        //Commit the benchmark

        sqlx::query!("INSERT INTO Reporter (host_id, timestamp, commit) VALUES (?, ?, ?)", 
        id,
        &report.timestamp,
        &report.commit).execute(&db).await?;
        let id = sqlx::query!("SELECT LAST_INSERT_ID() as id").fetch_one(&db).await?.id;

        for (name, time) in report.metrics {
            sqlx::query!("INSERT INTO Metrics (reporter_id, name, time) VALUES (?, ?, ?)", 
            id,
            &name,
            &time).execute(&db).await?;
        }

        trans.commit().await?;
        db.close().await;

        //Push data
        Ok(())
    }
}
