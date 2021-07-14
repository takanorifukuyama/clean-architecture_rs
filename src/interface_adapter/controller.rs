use tonic::{Request, Response, Status};

use user::user_api_server::UserApi;
use user::{CreateRequest, CreateResponse};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug)]
pub struct UserHandler {}

impl UserHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl UserApi for UserHandler {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        println!("Got a request piyo");

        let response = CreateResponse {
            message: format!("PiyoPiyo {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(response))
    }
}
