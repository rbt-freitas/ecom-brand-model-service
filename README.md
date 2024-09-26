# Ecommerce Brand and Model Management API

## Project Description

This service is part of a set of components created by me with the aim of studying the language 'RUST' and 
the software architecture. Using the concept of Domain-Driven-Design it offers the 'CRUD' operations for the Brand and Model entities.

## Prerequisites
Before you begin, ensure you have met the following requirements:
- **Rust**: You need to have Rust installed on your machine. You can download and install Rust from rust-lang.org.
- **Postgres**: You need to have Postgres installed and running. You can download and install XXXXXXXXXXXXX from XXXXXXXXXXXXXXX.
- **Docker**: Or you can use Docker installed to run RabbitMQ and MongoDB in containers. You can download and install Docker from docker.com.

## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/rbt-freitas/ecom-brand-model-service.git
    cd ecom-brand-model-service
    ```

2. Set up the environment variables:
    Create a `.env` file in the root of the project and add the following:
    ```env
    AMQP_ADDR=amqp://127.0.0.1:5672/%2f
    MONGODB_URI=mongodb://your_username:your_password@localhost:27017
    ```

    Or copy the file `.env.example` to `.env` and change the <your_username> and <your_password>:
    ```bash
    cp .env.example .env
    ```

3. Build the project:
    ```sh
    cargo build
    ```

## Running RabbitMQ and MongoDB with Docker
To run RabbitMQ and MongoDB using Docker, you can use the following commands:

1. **RabbitMQ**:
    ```sh
    docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management
    ```

2. **MongoDB**:
    ```sh
    docker run -d \
        --name mongodb \
        -e MONGO_INITDB_ROOT_USERNAME=<your_username> \
        -e MONGO_INITDB_ROOT_PASSWORD=<your_password> \
        -p 27017:27017 \
        mongo
    ```


## Usage
To run the project, use the following command: cargo run
    ```sh
    cargo run 
    ```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.