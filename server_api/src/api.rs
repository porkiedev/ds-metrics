//
// Contains the code pertaining to the API
//

use tonic::{Request, Response, Status};
use tracing::debug;

// Include the protobuf definitions
tonic::include_proto!("api");

pub struct Server {}
#[tonic::async_trait]
impl core_service_server::CoreService for Server {
    async fn test(&self, request: Request<Null>) -> Result<Response<Null>, Status> {
        
        debug!("Test function called");
        
        Err(Status::ok("It worked! :)"))
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        todo!()
    }
}
