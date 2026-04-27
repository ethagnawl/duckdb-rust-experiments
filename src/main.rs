use duckdb::{Connection, Result};

// Use cases:
// - logs
// - exports from legacy systems
// - prototyping
// - analytics

#[derive(Debug)]
struct JobSummary {
    jobs_processed: Option<i64>,
    // by default, duckdb is strict about column values and presence
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    // specifying columns is optimistic and will auto-populate nulls if key missing
    // ignore_errors will suppress errors when: "jobs_processed": "seven" iff column is spec'd
    // integer auto-casts for "66"
    conn.execute_batch(
        "
        CREATE TABLE job_summaries AS SELECT jobs_processed FROM read_json_auto('data/*.json', columns = {jobs_processed: 'INTEGER'}, ignore_errors = true),
        "
    )?;

    conn.execute_batch(
        "
        PRAGMA threads=10;
    ",
    )?;

    let mut stmt =
        conn.prepare("SELECT jobs_processed FROM job_summaries where jobs_processed not null")?;
    let jobs_processeds: Vec<i64> = stmt
        .query_map([], |row| row.get::<_, i64>(0))?
        .collect::<duckdb::Result<Vec<_>>>()?;

    let vec_jobs_processeds: Vec<JobSummary> = jobs_processeds
        .into_iter()
        .map(|x| JobSummary {
            jobs_processed: Some(x),
        })
        .collect();
    for jobs_processed in vec_jobs_processeds {
        println!(
            "jobs_processed: {}",
            jobs_processed.jobs_processed.unwrap_or_default()
        );
    }

    let total_jobs_processed: i64 = conn.query_row(
        "SELECT sum(jobs_processed) FROM job_summaries where jobs_processed not null",
        [],
        |row| row.get(0),
    )?;

    println!("total_jobs_processed: {}", total_jobs_processed);

    Ok(())
}
