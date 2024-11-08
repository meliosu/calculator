use calculator::{calculator_server::CalculatorServer, CalculatorService};
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    Server::builder()
        .add_service(CalculatorServer::new(CalculatorService))
        .serve("127.0.0.1:4800".parse().unwrap())
        .await
        .unwrap();
}

mod calculator {
    use calculator_server::Calculator;
    use operand::Value;
    use tonic::{async_trait, Request, Response, Status};

    type ServerResult<T> = Result<Response<T>, Status>;

    tonic::include_proto!("calculator");

    #[derive(Clone)]
    pub struct CalculatorService;

    #[async_trait]
    impl Calculator for CalculatorService {
        async fn calculate(
            &self,
            request: Request<OperationRequest>,
        ) -> ServerResult<OperationResponse> {
            use Value::{Float, Integer};

            let request = request.into_inner();

            let Some(lhs) = request.lhs.and_then(|v| v.value) else {
                return Err(Status::invalid_argument("invalid argument"));
            };

            let Some(rhs) = request.rhs.and_then(|v| v.value) else {
                return Err(Status::invalid_argument("invalid argument"));
            };

            let result = match (request.operation(), lhs, rhs) {
                (OperationType::Add, Integer(lhs), Integer(rhs)) => Integer(lhs + rhs),
                (OperationType::Sub, Integer(lhs), Integer(rhs)) => Integer(lhs - rhs),
                (OperationType::Mul, Integer(lhs), Integer(rhs)) => Integer(lhs * rhs),
                (OperationType::Div, Integer(lhs), Integer(rhs)) => Integer(lhs / rhs),
                (OperationType::Add, Float(lhs), Float(rhs)) => Float(lhs + rhs),
                (OperationType::Sub, Float(lhs), Float(rhs)) => Float(lhs - rhs),
                (OperationType::Mul, Float(lhs), Float(rhs)) => Float(lhs * rhs),
                (OperationType::Div, Float(lhs), Float(rhs)) => Float(lhs / rhs),

                _ => return Err(Status::invalid_argument("invalid argument")),
            };

            let response = Response::new(OperationResponse {
                answer: Some(Operand {
                    value: Some(result),
                }),
            });

            Ok(response)
        }
    }
}
