use tonic::{Request, Response, Status};

use crate::tonic_proto_out::basic_service_server::BasicService;
use crate::tonic_proto_out::{HealthCheckRequest, HealthCheckResponse};

#[derive(Default)]
pub struct BasicRouter {}

#[tonic::async_trait]
impl BasicService for BasicRouter {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            msg: "Ok".to_string(),
        }))
    }
}
