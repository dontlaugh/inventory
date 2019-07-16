# inventory

Print stuff across AWS accounts.

## Configuration

By default, a TOML config will be searched for at **~/.config/inventory/config.toml**.

```toml
[[aws_context]]
region = "us-east-1"
role = "role_to_assume"
account = "987654321011"

[[aws_context]]
region = "us-east-1"
role = "role_to_assume"
account = "123456789000"
```

