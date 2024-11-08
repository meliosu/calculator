use calculator::{
    calculator_client::CalculatorClient, operand::Value, Operand, OperationRequest, OperationType,
};
use tonic::Request;

#[tokio::main]
async fn main() {
    let mut client = CalculatorClient::connect("http://127.0.0.1:4800")
        .await
        .unwrap();

    let request = Request::new(OperationRequest {
        operation: OperationType::Mul.into(),
        lhs: Some(Operand {
            value: Some(Value::Float(2.0)),
        }),
        rhs: Some(Operand {
            value: Some(Value::Float(2.5)),
        }),
    });

    let response = client.calculate(request).await.unwrap();

    println!("{response:#?}");
}

mod calculator {
    tonic::include_proto!("calculator");
}
