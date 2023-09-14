use crate::{bsp::partition::Bucket, util::lerp};

use super::{
    entry::Entry,
    partition::{Dimension, Partition}, aabb::Aabb,
};

const MAX_NUM_ENTRIES: usize = 2;

pub struct Children {
    pub left_child: Box<Node>,
    pub right_child: Box<Node>,
}

pub struct Node {
    pub entries: Vec<Entry>,
    pub entries_aabb: Option<Aabb>,
    pub children: Option<Children>,
    pub partition: Option<Partition>,
}

impl Node {
    pub fn new_with(entries: Vec<Entry>, dimension: Dimension) -> Self {
        if entries.len() <= MAX_NUM_ENTRIES {
            let entries_aabb = Aabb::merged(entries.iter().map(|e| e.aabb));

            return Self {
                entries,
                entries_aabb,
                children: None,
                partition: None,
            };
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        match dimension {
            Dimension::X => {
                for entry in &entries {
                    min = min.min(entry.aabb.left());
                    max = max.max(entry.aabb.right());
                }
            }
            Dimension::Y => {
                for entry in &entries {
                    min = min.min(entry.aabb.bottom());
                    max = max.max(entry.aabb.top());
                }
            }
        };

        debug_assert!(min != f64::MAX);
        debug_assert!(max != f64::MIN);

        let partition = Partition {
            dimension,
            value: lerp(min, max, 0.5),
        };

        let mut left_entries = vec![];
        let mut mid_entries = vec![];
        let mut right_entries = vec![];

        let mut left_count = 0;
        let mut mid_count = 0;
        let mut right_count = 0;

        for entry in &entries {
            let bucket = partition.classify(&entry.aabb);
            match bucket {
                Bucket::Below => left_count += 1,
                Bucket::Intersecting => mid_count += 1,
                Bucket::Above => right_count += 1,
            }
        }

        left_entries.reserve(left_count);
        mid_entries.reserve(mid_count);
        right_entries.reserve(right_count);

        for entry in entries {
            let bucket = partition.classify(&entry.aabb);
            match bucket {
                Bucket::Below => left_entries.push(entry),
                Bucket::Intersecting => mid_entries.push(entry),
                Bucket::Above => right_entries.push(entry),
            }
        }

        let entries_aabb = Aabb::merged(mid_entries.iter().map(|e| e.aabb));
        match dimension {
            Dimension::X => {
                mid_entries.sort_unstable_by(|a, b| a.aabb.top().partial_cmp(&b.aabb.top()).unwrap());
            },
            Dimension::Y => {
                mid_entries.sort_unstable_by(|a, b| a.aabb.right().partial_cmp(&b.aabb.right()).unwrap());
            },
        }

        Self {
            entries: mid_entries,
            entries_aabb,
            children: Some(Children {
                left_child: Box::new(Self::new_with(left_entries, -dimension)),
                right_child: Box::new(Self::new_with(right_entries, -dimension)),
            }),
            partition: Some(partition),
        }
    }
}
