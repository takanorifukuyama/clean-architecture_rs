use tonic::transport::Server;

use clean_architecture_rs::interface_adapter::controller::{
    user::user_api_server::UserApiServer, UserHandler,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let jsr = UserHandler::new();
    Server::builder()
        .add_service(UserApiServer::new(jsr))
        .serve(addr)
        .await?;
    Ok(())
}
