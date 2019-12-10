use crate::seeker::{DocItem, RustDoc, TypeItem};
use quick_js::Context;
use serde_derive::Deserialize;
use serde_json::{self, Value};
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use string_cache::DefaultAtom as Atom;

#[derive(Clone, Debug, Deserialize)]
struct Parent {
    ty: usize,
    name: Atom,
}

#[derive(Debug, Deserialize)]
struct IndexItem {
    ty: usize,
    name: Atom,
    path: Atom,
    desc: Atom,
    #[serde(skip_deserializing)]
    parent: Option<Parent>,
    parent_idx: Option<usize>,
    search_type: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct SearchIndex {
    doc: Atom,
    #[serde(rename = "i")]
    items: Vec<IndexItem>,
    #[serde(rename = "p")]
    paths: Vec<Parent>,
}

impl From<IndexItem> for DocItem {
    /// Convert an IndexItem to DocItem based on if parent exists.
    fn from(item: IndexItem) -> DocItem {
        let name = TypeItem::new(item.ty, item.name);
        let parent = item.parent.map(|x| TypeItem::new(x.ty, x.name));

        DocItem::new(name, parent, item.path, item.desc)
    }
}

impl FromStr for RustDoc {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json_data = eval_js(s);
        let data: HashMap<String, SearchIndex> = serde_json::from_str(&json_data).unwrap();
        let mut items = BTreeSet::new();

        for (_, index) in data {
            let mut last_path = Atom::from("");
            let parents = index.paths;

            for mut item in index.items {
                // if `path` is empty, the `path` is the same as previous one
                // Dirty trick to compress the file size
                if !item.path.is_empty() {
                    last_path = item.path;
                };

                item.path = last_path.clone();

                // parent_idx is the index of the item in SearchIndex.paths
                item.parent = item.parent_idx.map(|idx| parents[idx].clone());

                items.insert(DocItem::from(item));
            }
        }

        Ok(RustDoc::new(items))
    }
}

fn eval_js(data: &str) -> String {
    let context = Context::new().unwrap();
    // Error is allowed here, because `addSearchOptions` and `initSearch` are not defined here.
    let _ = context.eval(data);
    context
        .eval("JSON.stringify(searchIndex)")
        .unwrap()
        .into_string()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let data = r#"
        var N=null,E="",T="t",U="u",searchIndex={};
        var R=["duration","implfuture","result","Output","The type of value produced on completion.","Future","async_std","An error returned when an operation could not be completedвЂ¦","A locked reference to the Stderr handle."];
        searchIndex["async_std"]={"doc":"Async version of the Rust standard library","i":[[23,"main",R[6],"Enables an async main function.",N,N],[23,"test",E,"Enables an async test function.",N,N]],
        "p":[[8,R[1]],[8,R[2]],[6,R[3]],[6,"SeekFrom"]]};
        addSearchOptions(searchIndex);initSearch(searchIndex);
        "#;
        let index = RustDoc::from_str(data).unwrap();
        let value = format!("{:?}", index);
        assert_eq!(value, "RustDoc { items: {DocItem { name: AttributeMacro(Atom('main' type=inline)), parent: None, path: Atom('async_std' type=dynamic), desc: Atom('Enables an async main function.' type=dynamic) }, DocItem { name: AttributeMacro(Atom('test' type=inline)), parent: None, path: Atom('async_std' type=dynamic), desc: Atom('Enables an async test function.' type=dynamic) }} }")
    }

    #[test]
    fn test_eval_js() {
        let data = r#"
        var N=null,E="",T="t",U="u",searchIndex={};
        var R=["duration","implfuture","result","Output","The type of value produced on completion.","Future","async_std","An error returned when an operation could not be completedвЂ¦","A locked reference to the Stderr handle."];
        searchIndex["async_std"]={"doc":"Async version of the Rust standard library","i":[[23,"main",R[6],"Enables an async main function.",N,N],[23,"test",E,"Enables an async test function.",N,N]],
        "p":[[8,R[5]],[8,R[1]],[4,R[2]],[4,"SeekFrom"]]};
        addSearchOptions(searchIndex);initSearch(searchIndex);
        "#;
        assert_eq!(eval_js(data),
            r#"{"async_std":{"doc":"Async version of the Rust standard library","i":[[23,"main","async_std","Enables an async main function.",null,null],[23,"test","","Enables an async test function.",null,null]],"p":[[8,"Future"],[8,null],[4,null],[4,"SeekFrom"]]}}"#
        );
    }
}
