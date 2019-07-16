use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AwsError {
    message: String,
}

impl fmt::Display for AwsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for AwsError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None // todo
    }
}

impl From<rusoto_core::request::TlsError> for AwsError {
    fn from(err: rusoto_core::request::TlsError) -> Self {
        AwsError {
            message: err.description().to_owned(),
        }
    }
}

impl From<rusoto_elb::DescribeLoadBalancersError> for AwsError {
    fn from(err: rusoto_elb::DescribeLoadBalancersError) -> Self {
        AwsError {
            message: err.description().to_owned(),
        }
    }
}

impl From<rusoto_elb::DescribeInstanceHealthError> for AwsError {
    fn from(err: rusoto_elb::DescribeInstanceHealthError) -> Self {
        AwsError {
            message: err.description().to_owned(),
        }
    }
}

impl From<rusoto_credential::CredentialsError> for AwsError {
    fn from(err: rusoto_credential::CredentialsError) -> Self {
        AwsError {
            message: err.description().to_owned(),
        }
    }
}
