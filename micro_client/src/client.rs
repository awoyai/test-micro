use micro_server::server::user_center::user_client::UserClient;
use micro_server::server::user_center::LoginRequest;
use tonic_example::server::trading::trade_client::TradeClient;
use tonic_example::server::trading::TradeRequest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(LoginRequest {
        name: "Tonic".into(),
        password: "pwd".into(),
    });

    let response = client.login(request).await?;

    println!("RESPONSE={:?}", response);

    let mut trade_cli = TradeClient::connect("http://[::1]:50051").await?;

    let trade_req = tonic::Request::new(TradeRequest {
        from: "wy".into(),
        to: "cyy".into(),
        amount: "100000".into(),
    });

    let trade_resp = trade_cli.trade(trade_req).await?;

    println!("RESPONSE={:?}", trade_resp);

    Ok(())
}