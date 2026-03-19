//! Mock HTTP servers for cloud provider integration tests
//!
//! Provides mock servers for AWS, Azure, and GCP APIs to enable testing
//! without requiring real cloud credentials or making actual API calls.

#[cfg(feature = "cloud-sdk")]
use mockito::{Mock, Server, ServerGuard};

#[cfg(feature = "cloud-sdk")]
/// Mock AWS EC2 API server
pub struct MockAwsEc2Server {
    server: ServerGuard,
}

#[cfg(feature = "cloud-sdk")]
impl MockAwsEc2Server {
    /// Create a new mock AWS EC2 server
    pub async fn new() -> Self {
        Self {
            server: Server::new_async().await,
        }
    }

    /// Get the base URL of the mock server
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Mock successful EC2 RunInstances response
    pub async fn mock_run_instances_success(&mut self) -> Mock {
        self.server
            .mock("POST", "/")
            .match_query(mockito::Matcher::Regex("Action=RunInstances".to_string()))
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<RunInstancesResponse>
    <instancesSet>
        <item>
            <instanceId>i-1234567890abcdef0</instanceId>
            <instanceState>
                <code>0</code>
                <name>pending</name>
            </instanceState>
        </item>
    </instancesSet>
</RunInstancesResponse>"#,
            )
            .create_async()
            .await
    }

    /// Mock AWS error response
    pub async fn mock_error(&mut self, status: usize, error_code: &str, message: &str) -> Mock {
        self.server
            .mock("POST", "/")
            .with_status(status)
            .with_header("content-type", "text/xml")
            .with_body(format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<ErrorResponse>
    <Error>
        <Code>{}</Code>
        <Message>{}</Message>
    </Error>
</ErrorResponse>"#,
                error_code, message
            ))
            .create_async()
            .await
    }
}

#[cfg(feature = "cloud-sdk")]
/// Mock AWS ECS API server
pub struct MockAwsEcsServer {
    server: ServerGuard,
}

#[cfg(feature = "cloud-sdk")]
impl MockAwsEcsServer {
    /// Create a new mock AWS ECS server
    pub async fn new() -> Self {
        Self {
            server: Server::new_async().await,
        }
    }

    /// Get the base URL of the mock server
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Mock successful ECS RunTask response
    pub async fn mock_run_task_success(&mut self) -> Mock {
        self.server
            .mock("POST", "/")
            .match_header("x-amz-target", "AmazonEC2ContainerServiceV20141113.RunTask")
            .with_status(200)
            .with_header("content-type", "application/x-amz-json-1.1")
            .with_body(
                r#"{
    "tasks": [
        {
            "taskArn": "arn:aws:ecs:us-east-1:123456789012:task/my-cluster/1234567890123456789",
            "taskDefinitionArn": "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:1",
            "lastStatus": "PENDING",
            "desiredStatus": "RUNNING"
        }
    ]
}"#,
            )
            .create_async()
            .await
    }
}

#[cfg(feature = "cloud-sdk")]
/// Mock Azure REST API server
pub struct MockAzureServer {
    server: ServerGuard,
}

#[cfg(feature = "cloud-sdk")]
impl MockAzureServer {
    /// Create a new mock Azure server
    pub async fn new() -> Self {
        Self {
            server: Server::new_async().await,
        }
    }

    /// Get the base URL of the mock server
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Mock Azure Managed Identity token endpoint
    pub async fn mock_managed_identity_token(&mut self) -> Mock {
        self.server
            .mock("GET", "/metadata/identity/oauth2/token")
            .match_query(mockito::Matcher::Regex(
                "api-version=2018-02-01".to_string(),
            ))
            .match_header("Metadata", "true")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
    "access_token": "mock-azure-access-token",
    "expires_on": "9999999999",
    "token_type": "Bearer"
}"#,
            )
            .create_async()
            .await
    }

    /// Mock Azure VM Scale Set creation
    pub async fn mock_vmss_creation(
        &mut self,
        subscription_id: &str,
        resource_group: &str,
    ) -> Mock {
        self.server
            .mock(
                "PUT",
                format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}",
                    subscription_id,
                    resource_group,
                    "test-vmss"
                )
                .as_str(),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
    "id": "/subscriptions/test-sub/resourceGroups/test-rg/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmss",
    "name": "test-vmss",
    "type": "Microsoft.Compute/virtualMachineScaleSets",
    "location": "eastus"
}"#,
            )
            .create_async()
            .await
    }
}

#[cfg(feature = "cloud-sdk")]
/// Mock GCP REST API server
pub struct MockGcpServer {
    server: ServerGuard,
}

#[cfg(feature = "cloud-sdk")]
impl MockGcpServer {
    /// Create a new mock GCP server
    pub async fn new() -> Self {
        Self {
            server: Server::new_async().await,
        }
    }

    /// Get the base URL of the mock server
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Mock GCP metadata server token endpoint
    pub async fn mock_metadata_token(&mut self) -> Mock {
        self.server
            .mock(
                "GET",
                "/computeMetadata/v1/instance/service-accounts/default/token",
            )
            .match_header("Metadata-Flavor", "Google")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
    "access_token": "mock-gcp-access-token",
    "expires_in": 3600,
    "token_type": "Bearer"
}"#,
            )
            .create_async()
            .await
    }

    /// Mock Google OAuth2 token endpoint for service account JWT exchange
    pub async fn mock_oauth2_token(&mut self) -> Mock {
        self.server
            .mock("POST", "/oauth2/v4/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
    "access_token": "mock-gcp-service-account-token",
    "expires_in": 3600,
    "token_type": "Bearer"
}"#,
            )
            .create_async()
            .await
    }

    /// Mock GCP Compute Engine instance creation
    pub async fn mock_compute_instance_creation(&mut self, project: &str, zone: &str) -> Mock {
        self.server
            .mock(
                "POST",
                format!("/compute/v1/projects/{}/zones/{}/instances", project, zone).as_str(),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
    "id": "1234567890123456789",
    "name": "test-instance",
    "zone": "us-central1-a",
    "status": "PROVISIONING"
}"#,
            )
            .create_async()
            .await
    }
}
