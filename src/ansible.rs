#![allow(dead_code)]
use serde::Serialize;
use serde_json::{json, Value};

/*

some links:
https://stackoverflow.com/questions/49783039/how-to-create-custom-json-structure-ansible-inventory
https://docs.ansible.com/ansible/latest/dev_guide/developing_inventory.html#developing-inventory-scripts
https://docs.ansible.com/ansible/latest/plugins/inventory/script.html

*/

const ALL: &'static str = "all";
const UNGROUPED: &'static str = "ungrouped";
const META: &'static str = "_meta";

struct Inventory {
    data: Value,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            data: json!({
                META: {
                  "hostvars": {}
                },
                ALL: {
                  "children": [
                    UNGROUPED
                  ]
                },
                UNGROUPED: {
                  "children": [
                  ]
                }
            }),
        }
    }

    /// Add the group if it does not exist.
    pub fn add_group(&mut self, group: &str) {
        // check if group exists
        let g = &self.data[group];
        if *g != Value::Null {
            return;
        }

        // vars is an object, children is a list of string which must
        // correspond to a top-level group name key.
        self.data[group] = json!({ "children": [], "vars": {} });

        // every group needs to be added to the "all" group's children
        assert!(self.data[ALL]["children"].is_array());
        self.data[ALL]["children"]
            .as_array_mut()
            .unwrap()
            .push(Value::String(group.to_string()));
    }

    /// Sets a group var. Adds the group if it does not exist.
    pub fn add_group_var<T>(&mut self, group: &str, key: &str, value: T)
    where
        T: Into<Value> + Serialize,
    {
        self.add_group(group);
        self.data[group]["vars"][key] = value.into();
    }

    // TODO: pub fn add_group_vars

    // pub fn add_group_child

    pub fn add_host(&mut self, group: Option<&str>, host: &str) {
        let group = group.unwrap_or(UNGROUPED);

        self.add_group(group);
        let hosts = &self.data[group]["hosts"];
        if *hosts == Value::Null {
            self.data[group]["hosts"] = json!([]);
        }
        assert!(self.data[group]["hosts"].is_array());
        self.data[group]["hosts"]
            .as_array_mut()
            .unwrap()
            .push(Value::String("host".to_string()));
    }

    // pub fn add_host_var

    pub fn to_string(&self) -> String {
        self.data.to_string()
    }
}

#[test]
fn test_new() {
    let i = Inventory::new();

    let data: Value = serde_json::from_str(EMPTY).unwrap();
    let data1: Value = serde_json::from_str(&i.to_string()).unwrap();
    assert_eq!(data, data1);
}

#[test]
fn test_add_group() {
    let mut i = Inventory::new();
    i.add_group("foo");
    i.add_group("baz");
    i.add_group("foo");

    // the order of the keys doesn't matter
    let expected_str = r#"
    {
        "_meta": {
            "hostvars": {}
        },
        "baz": {
            "children": [],
            "vars": {}
        },
        "foo":  {
            "children": [],
            "vars": {}
        },
        "all": {
            "children": [
            "ungrouped",
            "foo",
            "baz"
            ]
        },
        "ungrouped": {
            "children": [
            ]
        }
    }
    "#;
    let expected: Value = serde_json::from_str(expected_str).unwrap();
    let actual: Value = serde_json::from_str(&i.to_string()).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_add_group_var() {
    let mut i = Inventory::new();
    i.add_group_var("foo", "no", 69);
    i.add_group_var("baz", "hello", "world");
    i.add_group_var("baz", "favorite", 100);
    i.add_group_var("foo", "in", "the");
    i.add_group_var("foo", "champagne", "room");

    let expected_str = r#"
    {
        "_meta": {
            "hostvars": {}
        },
        "baz": {
            "children": [],
            "vars": {
                "hello": "world",
                "favorite": 100
            }
        },
        "foo":  {
            "children": [],
            "vars": {
                "no": 69,
                "in": "the",
                "champagne": "room"
            }
        },
        "all": {
            "children": [
            "ungrouped",
            "foo",
            "baz"
            ]
        },
        "ungrouped": {
            "children": [
            ]
        }
    }
    "#;
    let expected: Value = serde_json::from_str(expected_str).unwrap();
    let actual: Value = serde_json::from_str(&i.to_string()).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_add_host() {
    let expected_str = r#"
    {
        "_meta": {
            "hostvars": {}
        },
        "all": {
            "children": [
            "ungrouped"
            ]
        },
        "ungrouped": {
            "children": [
            ]
        }
    }
    "#;
}
/// Empty is the minimum valid json for ansible inventory
#[cfg(test)]
const EMPTY: &'static str = r#"
    {
        "_meta": {
            "hostvars": {}
        },
        "all": {
            "children": [
            "ungrouped"
            ]
        },
        "ungrouped": {
            "children": [
            ]
        }
    }
    "#;
