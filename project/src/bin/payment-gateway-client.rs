use payments::payment_gateway_client::PaymentGatewayClient;
use payments::SendPaymentRequest;


pub mod payments {
    tonic::include_proto!("payments");
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentGatewayClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(SendPaymentRequest {
        from_address: "10000000".to_owned(),
        to_address: "1234567".into(),
        amount: 100,
    });
    let response = client.send_payment(request).await?;
    println!("RESPONSE={:?}", response);
    
    Ok(())
}