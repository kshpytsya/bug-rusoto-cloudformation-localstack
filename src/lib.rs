#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn describe_stacks() {
        use rusoto_cloudformation::CloudFormation;

        let region = if let Ok(endpoint) = std::env::var("LOCALSTACK_ENDPOINT") {
            rusoto_core::Region::Custom {
                name: "us-east-1".to_owned(),
                endpoint: endpoint.to_owned(),
            }
        } else {
            rusoto_core::Region::default()
        };

        let credentials = rusoto_credential::DefaultCredentialsProvider::new().unwrap();

        let client = rusoto_cloudformation::CloudFormationClient::new_with(
            rusoto_core::HttpClient::new().unwrap(),
            credentials,
            region.clone(),
        );

        client
            .describe_stacks(rusoto_cloudformation::DescribeStacksInput {
                next_token: None,
                stack_name: None,
            })
            .await
            .unwrap();
    }
}
