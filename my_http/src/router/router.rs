use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Weak, Rc};

#[derive(Debug)]
struct Node {
    next: RefCell<Box<HashMap<String, Box<Node>>>>,
    handler: fn(),
    is_end: bool, // 是否是匹配终点的Node
    index: u64,
}

#[derive(Debug)]
struct Item<'a> {
    uri: &'a str,// (匹配的uri, 动态路由代表是param)
    is_extra: bool,// (是否*全局匹配)
    is_dyn: bool,// (是否为动态路由)
    index: u64,// (该段uri在整段uri的vec索引)
}

struct Value {
    query: HashMap<String, String>,
    params: HashMap<String, String>,
    handler: fn (),
}

struct Router {}

impl Router {
    fn new() -> RouterTree {
        RouterTree {
            tree: HashMap::new()
        }
    }
}

#[derive(Debug)]
struct RouterTree {
    tree: HashMap<String, RefCell<HashMap<String, Box<Node>>>>,
}

impl RouterTree {
    // 添加节点的方法
    fn add<'a>(&self, method: &'a str, uri: &'a str, handler: fn ()) -> &RouterTree {
        let items = into_items(uri);
        for item in items.iter() {
            println!("{:?}", item);
        }
        self
    }
    // 查找节点的方法
    fn find(&self, method: String, uri: String) -> Value {
        Value{
            query: Default::default(),
            params: Default::default(),
            handler: || {

            }
        }
    }
}

fn into_items<'a>(uri: &'a str) -> Vec<Item<'a>> {
    let mut items: Vec<Item<'a>> = vec![];
    let mut index: u64 = 0;
    let mut col = uri.split('/').collect::<Vec<&'a str>>();
    col.remove(0);
    let mut col = col.iter();
    for &uri in col {
        items.push(create_item(uri, index));
        index += 1;
    }
    items
}

fn create_node(handler: fn(), is_end: bool, index: u64) -> Node {
    Node {
        next: RefCell::new(Box::new(Default::default())),
        handler,
        is_end,
        index
    }
}

fn create_item(uri: &str, index: u64) -> Item {
    if uri.starts_with("*") {
        Item {
            uri: &uri[1..],
            is_extra: true,
            is_dyn: false,
            index,
        }
    } else if uri.starts_with(":") {
        Item {
            uri: &uri[1..],
            is_extra: false,
            is_dyn: true,
            index,
        }
    } else {
        Item {
            uri,
            is_extra: false,
            is_dyn: false,
            index,
        }
    }
}

pub fn test() {
    println!("router finished 10%");
    // let router = Router::new();
    // router.add("get", "/id/:id/*info", ||{
    //     println!("/id/:id/*info");
    // });
}
