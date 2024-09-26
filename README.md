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
    DATABASE_URL=postgres://<postgres_username>:<postgres_password>@127.19.0.2:5432/<database_name>
    HOST=127.0.0.1
    PORT=8080
    LOG_LEVEL=INFO
    ```

    Or copy the file `.env.example` to `.env` and change the <your_username> and <your_password>:
    ```bash
    cp .env.example .env
    ```

3. Build the project:
    ```sh
    cargo build
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