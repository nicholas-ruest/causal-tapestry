//! Executed RVF append-only evidence bundle adapter.
use ct_errors::{Error, Result};
use rvf_runtime::{QueryOptions, RvfOptions, RvfStore};
use std::path::Path;

pub fn persist_vectors(path: &Path, values: &[f32]) -> Result<()> {
    if values.is_empty() {
        return Err(Error::Evidence("RVF evidence vector set is empty".into()));
    }
    let options = RvfOptions {
        dimension: 1,
        ..Default::default()
    };
    let mut store = RvfStore::create(path, options)
        .map_err(|error| Error::Evidence(format!("RVF create: {error}")))?;
    let vectors: Vec<[f32; 1]> = values.iter().map(|value| [*value]).collect();
    let refs: Vec<&[f32]> = vectors.iter().map(<[f32; 1]>::as_slice).collect();
    let ids: Vec<u64> = (1..=u64::try_from(values.len())
        .map_err(|_| Error::Evidence("too many evidence records".into()))?)
        .collect();
    let result = store
        .ingest_batch(&refs, &ids, None)
        .map_err(|error| Error::Evidence(format!("RVF ingest: {error}")))?;
    if result.accepted != values.len() as u64 {
        return Err(Error::Evidence("RVF rejected an evidence vector".into()));
    }
    drop(store);
    let reopened = RvfStore::open_readonly(path)
        .map_err(|error| Error::Evidence(format!("RVF reopen: {error}")))?;
    let hits = reopened
        .query(&[values[0]], 1, &QueryOptions::default())
        .map_err(|error| Error::Evidence(format!("RVF query: {error}")))?;
    if hits.is_empty() {
        return Err(Error::Evidence("RVF readback returned no evidence".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_rvf_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        persist_vectors(&dir.path().join("evidence.rvf"), &[0.25, 0.75]).unwrap();
    }
}
