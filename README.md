# Rust gRPC Service with Tonic, Tokio, and other tools

This repository contains a Rust gRPC service implemented using Tonic, Tokio, and other tools. The service provides a payment gateway functionality and includes an example implementation of the `PaymentGateway` trait.

## Prerequisites

To build and run this service, you need to have the following dependencies installed:

- Rust (stable version)
- Tonic
- Tokio

## Installation

1. Clone this repository:

   ```shell
   git clone https://github.com/developeruche/simple-rust-grpc.git
   ```
2. Get into the project directory
3. Build the project

## Usage 
To start the gRPC service, run the following command:
   ```shell
   cargo run
   ```
The service will be available at localhost:50051 by default.

API
The service provides the following gRPC API methods:

SendPayment
Sends a payment request.

### Request
```proto
message SendPaymentRequest {
  string amount = 1;
  string to_address = 2;
}
```

### Response
```proto
message SendPaymentResponse {
  bool successful = 1;
  string message = 2;
}
```

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.


## License
This project is licensed under the MIT License.
