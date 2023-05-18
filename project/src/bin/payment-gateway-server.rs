use tonic::{transport::Server, Request, Response, Status};
use payments::payment_gateway_server::{PaymentGateway, PaymentGatewayServer};
use payments::{SendPaymentRequest, SendPaymentResponse};




pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct PaymentGatewayService {}

#[tonic::async_trait]
impl PaymentGateway for PaymentGatewayService {
    async fn send_payment(
        &self,
        request: Request<SendPaymentRequest>,
    ) -> Result<Response<SendPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let response = SendPaymentResponse {
            successful: true,
            message: format!("Sent a {} payment request to {}.", req.amount, req.to_address).into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let payment_gateway = PaymentGatewayService::default();

    Server::builder()
        .add_service(PaymentGatewayServer::new(payment_gateway))
        .serve(addr)
        .await?;

    Ok(())
}
