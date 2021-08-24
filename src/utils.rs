use std::ops::Bound;

use neon::{
    context::{Context, TaskContext},
    prelude::Handle,
    types::JsString,
};

pub fn to_bound_range(
    start: Option<Vec<u8>>,
    end: Option<Vec<u8>>,
    include_start: bool,
    include_end: bool,
) -> tikv_client::BoundRange {
    let start_bound = if let Some(start) = start {
        if include_start {
            Bound::Included(start)
        } else {
            Bound::Excluded(start)
        }
    } else {
        Bound::Unbounded
    };
    let end_bound = if let Some(end) = end {
        if include_end {
            Bound::Included(end)
        } else {
            Bound::Excluded(end)
        }
    } else {
        Bound::Unbounded
    };
    tikv_client::BoundRange::from((start_bound, end_bound))
}
