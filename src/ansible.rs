use serde_json;
use serde_json::json;

/*

some links:
https://stackoverflow.com/questions/49783039/how-to-create-custom-json-structure-ansible-inventory
https://docs.ansible.com/ansible/latest/dev_guide/developing_inventory.html#developing-inventory-scripts
https://docs.ansible.com/ansible/latest/plugins/inventory/script.html

goals:
* provide a default implmentatino
*

data structure:

```
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
```
*/

struct Inventory {
    data: serde_json::Value,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            data: json!({
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
            }),
        }
    }

    pub fn add_group(&mut self, group: &str) {
        // check if group exists
        let g = &self.data[group];
        if *g != serde_json::Value::Null {
            return;
        }

        // vars is an object, children is a list of string which must
        // correspond to a top-level group name key.
        self.data[group] = json!({ "children": [], "vars": {} });
    }

    // pub fn add_group_var

    // pub fn add_group_vars

    // pub fn add_group_child

    // pub fn add_host

    // pub fn add_host_var

    pub fn to_string(&self) -> String {
        self.data.to_string()
    }
}

#[test]
fn test_new() {
    let i = Inventory::new();

    let data: serde_json::Value = serde_json::from_str(EMPTY).unwrap();
    let data1: serde_json::Value = serde_json::from_str(&i.to_string()).unwrap();
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
            "ungrouped"
            ]
        },
        "ungrouped": {
            "children": [
            ]
        }
    }
    "#;
    let expected: serde_json::Value = serde_json::from_str(expected_str).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&i.to_string()).unwrap();
    assert_eq!(expected, actual);


    
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