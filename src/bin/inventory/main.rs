use clap::{App, Arg, SubCommand};
use failure::Error;
use inventory::aws::*;
use inventory::config::*;
use molt;
use rusoto_core::Region;
use rusoto_ec2::Instance;
use std::env;
use std::path::Path;
use std::str::FromStr;
use prettytable::{Table, row, cell};
use prettytable::format::TableFormat;

const VERSION: &'static str = "0.2.0";

fn main() -> Result<(), Error> {
    let default_dir = home_with(".config/inventory/");
    let default_config = home_with(".config/inventory/config.toml");
    create_dir(&default_dir.clone())?;
    let app = App::new("inventory")
        .about("print resources across multiple AWS accounts")
        .version(VERSION)
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .default_value(&default_config),
        )
        .subcommand(SubCommand::with_name("ec2").about("print EC2 instances"));

    let matches = app.get_matches();
    let config_path = matches.value_of("config").unwrap();
    let config = Config::from_file(&config_path)?;

    // Create Tcl interpreter
    let mut interp = molt::Interp::new();

    if let Some(_) = matches.subcommand_matches("ec2") {
        let mut table = Table::new();
        let mut format = TableFormat::new();
        format.column_separator('\t');
        table.set_format(format);
        table.set_titles(row!("Name", "ID", "Type", "Private IP", "AMI"));
        for ctx in config.aws_context {
            let region = Region::from_str(&ctx.region)?;
            let instances: Vec<Instance> = get_ec2_instances(region, ctx.account, ctx.role)?;
            for i in instances {
                let private_ip = i.private_ip_address.unwrap_or("<none>".to_string());
                let instance_type = i.instance_type.unwrap_or("UNKNOWN".to_string());
                let ami = i.image_id.unwrap_or("<none>".to_string());
                let _role = i.iam_instance_profile.map(|prof| prof.arn ).unwrap_or(Some("<none>".to_string())).unwrap();
                let sss = ctx.script.clone();
                if let Some(script) = sss {
                    // TODO: eval dynamic Tcl script to let people create custom tables
                    interp.set_var("private_ip", &molt::Value::from(private_ip));
                    interp.eval_body(&script).unwrap();
                    let result = interp.var("output").expect("molt result");
                    println!("{}", result);
                } else {
                    // Default formatting

                    let name = extract_tag_by_key(i.tags, "Name").unwrap_or("<none>".to_string());
                    let row = row!(name, i.instance_id.unwrap(), instance_type, private_ip, ami);
                    table.add_row(row);
                }
            }
        }
        table.printstd();
        return Ok(());
    }

    println!("error: must provide a subcommand");
    std::process::exit(1);
    #[allow(unreachable_code)]
    Ok(())
}

/// Join a path to the HOME directory. Panics on any error. HOME env var must be set.
fn home_with(path: &'static str) -> String {
    Path::new(&env::var("HOME").unwrap())
        .join(path)
        .to_str()
        .unwrap()
        .to_owned()
}

/// Create a directory if it does not exist.
fn create_dir<P: AsRef<Path>>(p: P) -> Result<(), Error> {
    if !p.as_ref().exists() {
        std::fs::create_dir_all(p.as_ref())?;
    }
    Ok(())
}
