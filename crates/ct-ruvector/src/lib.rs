//! Executed RuVector analogue index boundary.
use ct_embedding::DIMENSION;
use ct_errors::{Error, Result};
use ct_types::Analogue;
use ruvector_core::types::{DbOptions, DistanceMetric, SearchQuery, VectorEntry};
use ruvector_core::VectorDB;
use std::collections::HashMap;

pub struct AnalogueIndex {
    db: VectorDB,
    len: usize,
}

impl AnalogueIndex {
    pub fn new()->Result<Self>{
        let options=DbOptions{dimensions:DIMENSION,distance_metric:DistanceMetric::Euclidean,
            storage_path:"memory://causal-tapestry".to_owned(),hnsw_config:None,quantization:None};
        let db=VectorDB::new(options).map_err(|error|Error::Upstream(format!("RuVector create: {error}")))?;
        Ok(Self{db,len:0})
    }

    pub fn insert(&mut self, incident_id:&str, vector:Vec<f32>, outcome:&str)->Result<()>{
        if vector.len()!=DIMENSION { return Err(Error::InvalidInput("incident embedding dimension mismatch".into())); }
        let mut metadata=HashMap::new();
        metadata.insert("outcome".to_owned(),serde_json::json!(outcome));
        self.db.insert(VectorEntry{id:Some(incident_id.to_owned()),vector,metadata:Some(metadata)})
            .map_err(|error|Error::Upstream(format!("RuVector insert: {error}")))?;
        self.len+=1;
        Ok(())
    }

    pub fn search(&self, vector:&[f32], k:usize)->Result<Vec<Analogue>>{
        if self.len==0 { return Ok(Vec::new()); }
        let hits=self.db.search(SearchQuery{vector:vector.to_vec(),k:k.min(self.len),filter:None,ef_search:None})
            .map_err(|error|Error::Upstream(format!("RuVector search: {error}")))?;
        Ok(hits.into_iter().map(|hit|Analogue{incident_id:hit.id,distance:hit.score,
            outcome:hit.metadata.and_then(|m|m.get("outcome").and_then(|v|v.as_str()).map(str::to_owned)).unwrap_or_default()}).collect())
    }

    pub fn len(&self)->usize{self.len}
    pub fn is_empty(&self)->bool{self.len==0}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nearest_analogue_executes_ruvector() {
        let mut index=AnalogueIndex::new().unwrap();
        index.insert("near",vec![0.0;DIMENSION],"rolled back").unwrap();
        index.insert("far",vec![1.0;DIMENSION],"scaled").unwrap();
        assert_eq!(index.search(&vec![0.1;DIMENSION],1).unwrap()[0].incident_id,"near");
    }
}

