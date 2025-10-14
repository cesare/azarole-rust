use serde::Serialize;

use crate::models::{Workplace, WorkplaceId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct WorkplaceView<'a> {
    id: &'a WorkplaceId,
    name: &'a String,
}

impl<'a> WorkplaceView<'a> {
    pub(in crate::handlers) fn new(workplace: &'a Workplace) -> Self {
        Self {
            id: &workplace.id,
            name: &workplace.name,
        }
    }
}

