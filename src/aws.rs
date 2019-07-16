use failure::Fail;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::AutoRefreshingProvider;
use rusoto_ec2::{DescribeInstancesRequest, DescribeVpcsRequest, Ec2, Ec2Client, Instance, Vpc};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};

pub struct Arn;

impl Arn {
    pub fn make_string(account: String, role: String) -> String {
        format!("arn:aws:iam::{}:role/{}", account, role)
    }
}

fn credentials_provider(
    region: Region,
    account: String,
    role: String,
) -> Result<AutoRefreshingProvider<StsAssumeRoleSessionCredentialsProvider>, AwsError> {
    let sts = StsClient::new(region);
    let arn = Arn::make_string(account, role);
    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        arn,
        "default".to_owned(),
        None,
        None,
        None,
        None,
    );
    Ok(rusoto_credential::AutoRefreshingProvider::new(provider)?)
}

#[derive(Fail, Debug)]
pub enum AwsError {
    #[fail(display = "credentials error: {}", _0)]
    Credentials(#[cause] rusoto_credential::CredentialsError),

    #[fail(display = "tls error: {}", _0)]
    TlsError(#[cause] rusoto_core::request::TlsError),

    #[fail(display = "api error: {}", _0)]
    ApiError(String),
}

impl From<rusoto_credential::CredentialsError> for AwsError {
    fn from(err: rusoto_credential::CredentialsError) -> Self {
        AwsError::Credentials(err)
    }
}

impl From<rusoto_core::request::TlsError> for AwsError {
    fn from(err: rusoto_core::request::TlsError) -> Self {
        AwsError::TlsError(err)
    }
}

pub fn get_vpcs(region: Region, account: String, role: String) -> Result<Vec<Vpc>, AwsError> {
    let provider = credentials_provider(region.clone(), account, role)?;
    let client = Ec2Client::new_with(HttpClient::new()?, provider, region.clone());
    let req = DescribeVpcsRequest::default();
    let result = client
        .describe_vpcs(req)
        .sync()
        .map_err(|err| AwsError::ApiError(format!("{:?}", err)))?;
    // return an Ok(empty vec) if we get None, not an error
    Ok(result.vpcs.unwrap_or(Vec::new()))
}

pub fn get_ec2_instances(
    region: Region,
    account: String,
    role: String,
) -> Result<Vec<Instance>, AwsError> {
    let provider = credentials_provider(region.clone(), account, role)?;
    let client = Ec2Client::new_with(HttpClient::new()?, provider, region.clone());
    let req = DescribeInstancesRequest::default();
    let result = client
        .describe_instances(req)
        .sync()
        .map_err(|err| AwsError::ApiError(format!("{:?}", err)))?;
    let reservations: Vec<rusoto_ec2::Reservation> = result.reservations.unwrap_or(Vec::new());

    let mut instances = Vec::new();
    for res in reservations {
        instances.extend(res.instances.unwrap_or(Vec::new()));
    }

    Ok(instances)
}

pub fn extract_tag_by_key(tags: Option<Vec<rusoto_ec2::Tag>>, key: &str) -> Option<String> {
    match tags {
        Some(tags) => {
            for tag in tags {
                match tag.key {
                    Some(ref s) if s == key => {
                        return tag.value;
                    }
                    _ => continue,
                }
            }
        }
        None => {}
    }
    None
}
