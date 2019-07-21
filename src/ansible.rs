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

    // pub fn add_group

    // pub fn add_group_var

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

    let empty = r#"
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

    let data: serde_json::Value = serde_json::from_str(empty).unwrap();
    let data1: serde_json::Value = serde_json::from_str(&i.to_string()).unwrap();
    assert_eq!(data, data1);
}
