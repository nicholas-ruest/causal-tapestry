//! Hash-chained evidence ledger with RVF portability.
use ct_errors::{Error, Result};
use ct_rvf::persist_vectors;
use ct_types::EvidenceRecord;
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Default)]
pub struct Ledger {
    records: Vec<EvidenceRecord>,
}

impl Ledger {
    pub fn append(&mut self, kind: &str, payload: &str) -> Result<&EvidenceRecord> {
        if kind.is_empty() {
            return Err(Error::Evidence("evidence kind is empty".into()));
        }
        let previous_hash = self
            .records
            .last()
            .map(|record| record.hash.clone())
            .unwrap_or_else(|| "genesis".into());
        let sequence = self.records.len() as u64 + 1;
        let mut digest = Sha256::new();
        digest.update(sequence.to_le_bytes());
        digest.update(kind.as_bytes());
        digest.update(payload.as_bytes());
        digest.update(previous_hash.as_bytes());
        let hash = hex::encode(digest.finalize());
        self.records.push(EvidenceRecord {
            sequence,
            kind: kind.into(),
            payload: payload.into(),
            previous_hash,
            hash,
        });
        Ok(self.records.last().expect("record was appended"))
    }

    pub fn verify(&self) -> bool {
        let mut previous = "genesis".to_owned();
        for record in &self.records {
            let mut digest = Sha256::new();
            digest.update(record.sequence.to_le_bytes());
            digest.update(record.kind.as_bytes());
            digest.update(record.payload.as_bytes());
            digest.update(previous.as_bytes());
            if hex::encode(digest.finalize()) != record.hash {
                return false;
            }
            previous = record.hash.clone();
        }
        true
    }

    pub fn seal_rvf(&self, path: &Path) -> Result<String> {
        if !self.verify() {
            return Err(Error::Evidence("ledger chain verification failed".into()));
        }
        let values: Vec<f32> = self
            .records
            .iter()
            .map(|record| {
                u32::from_le_bytes([
                    record.hash.as_bytes()[0],
                    record.hash.as_bytes()[1],
                    record.hash.as_bytes()[2],
                    record.hash.as_bytes()[3],
                ]) as f32
                    / u32::MAX as f32
            })
            .collect();
        persist_vectors(path, &values)?;
        Ok(self
            .records
            .last()
            .map(|record| record.hash.clone())
            .unwrap_or_default())
    }

    pub fn records(&self) -> &[EvidenceRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chain_and_rvf_are_verified() {
        let mut ledger = Ledger::default();
        ledger.append("ingest", "three events").unwrap();
        ledger.append("decision", "deny").unwrap();
        assert!(ledger.verify());
        let dir = tempfile::tempdir().unwrap();
        assert!(!ledger
            .seal_rvf(&dir.path().join("ledger.rvf"))
            .unwrap()
            .is_empty());
    }
}
