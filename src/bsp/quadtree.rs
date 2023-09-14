use super::{aabb::Aabb, entry::Entry};

#[derive(Default, Clone)]
struct Node {
    entries: Vec<Entry>,
    aabb: Aabb,
    children: Option<Box<[Node; 4]>>,
}

pub struct Quadtree<const S: usize> {

    root: Box<Node>,
}

impl Node {
    pub fn new(entries: Vec<Entry>, depth: usize) -> Self {
        let aabb = Aabb::merged(entries.iter().map(|e| e.aabb));
        if entries.len() <= 3 || depth >= 5 {
            return Self {
                entries,
                aabb: aabb.unwrap_or_default(),
                children: None,
            };
        }

        if let Some(aabb) = aabb {
            let tl_aabb = Aabb::new(aabb.left(), aabb.center().x, aabb.top(), aabb.center().y);
            let tr_aabb = Aabb::new(aabb.center().x, aabb.right(), aabb.top(), aabb.center().y);
            let bl_aabb = Aabb::new(aabb.left(), aabb.center().x, aabb.center().y, aabb.bottom());
            let br_aabb = Aabb::new(
                aabb.center().x,
                aabb.right(),
                aabb.center().y,
                aabb.bottom(),
            );

            let mut tl_entries = vec![];
            let mut tr_entries = vec![];
            let mut bl_entries = vec![];
            let mut br_entries = vec![];

            for entry in entries.iter() {
                if entry.aabb.intersects(&tl_aabb) {
                    tl_entries.push(*entry);
                }
                if entry.aabb.intersects(&tr_aabb) {
                    tr_entries.push(*entry);
                }
                if entry.aabb.intersects(&bl_aabb) {
                    bl_entries.push(*entry);
                }
                if entry.aabb.intersects(&br_aabb) {
                    br_entries.push(*entry);
                }
            }

            let children: [Self; 4] = [
                Self::new(tl_entries, depth + 1),
                Self::new(tr_entries, depth + 1),
                Self::new(bl_entries, depth + 1),
                Self::new(br_entries, depth + 1),
            ];

            Self {
                entries: vec![],
                aabb,
                children: Some(Box::new(children)),
            }
        } else {
            Self {
                entries: vec![],
                aabb: Aabb::default(),
                children: None,
            }
        }
    }

    pub fn collect_collisions(&self, entry: &Entry, collisions: &mut Vec<Entry>) {
        if !entry.aabb.intersects(&self.aabb) {
            return;
        }

        if let Some(children) = &self.children {
            children[0].collect_collisions(entry, collisions);
            children[1].collect_collisions(entry, collisions);
            children[2].collect_collisions(entry, collisions);
            children[3].collect_collisions(entry, collisions);
        } else {
            collisions.extend(
                self.entries
                    .iter()
                    .filter(|e| e.aabb.intersects(&entry.aabb)),
            );
        }
    }
}

impl<const S: usize> Quadtree<S> {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            root: Box::new(Node::new(entries, 0)),
        }
    }

    pub fn collect_collisions(&self, entry: &Entry, collisions: &mut Vec<Entry>) {
        self.root.collect_collisions(entry, collisions);
    }
}
