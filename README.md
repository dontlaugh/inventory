# inventory

Print resources across multiple AWS accounts.

## Usage

```
inventory ec2
```

## Installation

You need Rust. And **~/.cargo/bin** must be on your PATH. Then run

```
cargo install --git https://github.com/dontlaugh/inventory
```

You may need to add `--force` to overwrite an existing binary.

Compiling the AWS libraries might make your computer take off like a jet engine for a few minutes. ;-)

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

## How it works

For each context configured, we

1. Assume the specified role using the "default" role (likely located at ~/.aws/credentials)
2. Run read-only "describe" commands to list resources
