use tonic::transport::Channel;

use crate::grpc_scheduler::{controller_client::ControllerClient, ActionRequest, ActionResponse};

pub struct GrpcSchedulerClient {
    client: ControllerClient<Channel>,
}

impl GrpcSchedulerClient {
    pub async fn new(grpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ControllerClient::connect(grpc_url.to_string()).await?;

        Ok(Self { client })
    }

    pub async fn schedule_action(
        &mut self,
        request: ActionRequest,
    ) -> Result<tonic::Streaming<ActionResponse>, Box<dyn std::error::Error>> {
        let response = self.client.schedule_action(request).await?.into_inner();

        Ok(response)
    }
}
