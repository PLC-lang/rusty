use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use serde::{Deserialize, Serialize};

use crate::ast::AstId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdProvider {
    current_id: Arc<AtomicUsize>,
}

impl IdProvider {
    pub fn next_id(&mut self) -> AstId {
        self.current_id.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for IdProvider {
    fn default() -> Self {
        IdProvider { current_id: Arc::new(AtomicUsize::new(1)) }
    }
}

#[cfg(test)]
mod id_tests {
    use super::IdProvider;

    #[test]
    fn id_provider_generates_unique_ids_over_clones() {
        let mut id1 = IdProvider::default();
        let mut id2 = id1.clone();

        assert_eq!(id1.next_id(), 1);
        assert_eq!(id2.next_id(), 2);
        assert_eq!(id1.next_id(), 3);
        assert_eq!(id2.next_id(), 4);
    }
}
