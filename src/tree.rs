use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Mutex;

use askama::Template;

use crate::menu::PasteMenu;
use crate::menu::PastePage;
use crate::models::ListRow;
use crate::models::Text;
use crate::models::TreeItem;

type ChildMap<'a> = HashMap<String, Vec<&'a TreeItem>>;

#[derive(Template)]
#[template(path = "tree/root.html")]
pub struct TreeRoot<'a> {
    list: &'a ListRow,
    items: &'a Vec<TreeItem>,
    mapping: ChildMap<'a>,
}

impl<'a> TreeRoot<'a> {
    pub fn new(list: &'a ListRow, items: &'a Vec<TreeItem>) -> Self {
        let mut mapping = ChildMap::with_capacity(items.len());

        for item in items {
            let parent = item.parent.decode().into_owned();
            let children = mapping.entry(parent).or_default();

            children.push(item);
        }

        Self {
            list,
            items,
            mapping,
        }
    }

    fn children(&self) -> Vec<TreeNode> {
        let children = self.mapping.get(TreeItem::ROOT);
        let breaker = Rc::new(CycleBreaker::new(self.items.len()));

        children
            .into_iter()
            .flatten()
            .filter(|target| breaker.check(target))
            .map(|target| TreeNode {
                breaker: breaker.clone(),
                mapping: &self.mapping,
                list: self.list,
                item: target,
            })
            .collect()
    }

    fn menu(&self) -> PasteMenu {
        PasteMenu::new(self.list, PastePage::Tree)
    }
}

struct CycleBreaker<'a>(Mutex<HashSet<&'a Text>>);

impl<'a> CycleBreaker<'a> {
    fn new(capacity: usize) -> Self {
        Self(Mutex::new(HashSet::with_capacity(capacity)))
    }

    fn check(&self, target: &'a TreeItem) -> bool {
        let mut set = self.0.lock().unwrap();

        set.insert(&target.id)
    }
}

#[derive(Template)]
#[template(path = "tree/node.html")]
struct TreeNode<'a> {
    breaker: Rc<CycleBreaker<'a>>,
    mapping: &'a ChildMap<'a>,
    list: &'a ListRow,
    item: &'a TreeItem,
}

impl<'a> TreeNode<'a> {
    fn children(&self) -> Vec<Self> {
        let id = &*self.item.id.decode();
        let children = self.mapping.get(id);
        let breaker = &self.breaker;

        children
            .into_iter()
            .flatten()
            .filter(|target| breaker.check(target))
            .map(|target| Self {
                breaker: breaker.clone(),
                mapping: self.mapping,
                list: self.list,
                item: target,
            })
            .collect()
    }
}
