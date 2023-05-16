use axum_grpc_consul::pb;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051";
    let calc = CalcService::default();
    println!("CalcServer 监听于 {}", addr);
    tonic::transport::Server::builder()
        .add_service(pb::calc_service_server::CalcServiceServer::new(
        calc,
        ))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}

#[derive(Default)]
pub struct CalcService {}

#[tonic::async_trait]
impl pb::calc_service_server::CalcService for CalcService {
    async fn add(
        &self,
        request: tonic::Request<pb::AddRequest>,
    ) -> Result<tonic::Response<pb::AddResponse>, tonic::Status> {
        let pb::CalculateRequset { x, y } = request.into_inner();
        Ok(tonic::Response::new(pb::AddResponse {
            result: x + y
        }))
    }
}
