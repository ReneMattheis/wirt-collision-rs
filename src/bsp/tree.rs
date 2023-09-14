use super::{
    entry::Entry,
    node::Node,
    partition::{Bucket, Dimension},
};

pub struct Tree {
    entries: Vec<Entry>,
    root: Node,
}

impl Tree {
    pub fn new_with(entries: Vec<Entry>) -> Self {
        Self {
            entries: entries.clone(),
            root: Node::new_with(entries, Dimension::X),
        }
    }

    pub fn internal_collisions(&self) -> Vec<(usize, usize)> {
        let mut collisions: Vec<(usize, usize)> = vec![];
        let mut nodes_to_search = vec![];
        for entry in &self.entries {
            nodes_to_search.clear();
            nodes_to_search.push(&self.root);

            while let Some(node) = nodes_to_search.pop() {
                let mut check_node_entries = true;

                if let Some(node_aabb) = node.entries_aabb {
                    check_node_entries = entry.aabb.intersects(&node_aabb);
                }

                if check_node_entries {
                    for other_entry in &node.entries {
                        if other_entry.id == entry.id {
                            continue;
                        }

                        if entry.aabb.intersects(&other_entry.aabb) {
                            collisions.push((entry.id, other_entry.id));
                        }
                    }
                }

                if let Some(children) = &node.children {
                    if let Some(partition) = &node.partition {
                        let partition_bucket = partition.classify(&entry.aabb);
                        match partition_bucket {
                            Bucket::Below => {
                                nodes_to_search.push(&children.left_child);
                            }
                            Bucket::Intersecting => {
                                nodes_to_search.push(&children.left_child);
                                nodes_to_search.push(&children.right_child);
                            }
                            Bucket::Above => {
                                nodes_to_search.push(&children.right_child);
                            }
                        }
                    }
                }
            }
        }

        collisions
    }
}
