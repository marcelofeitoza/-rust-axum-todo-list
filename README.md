# Rust Axum ToDoList App

## Overview

A simple and efficient todo list application built with Rust using the Axum framework. It provides a clean RESTful API to manage tasks, allowing users to add, update the status of, and delete tasks.

## Features

- Create new tasks with a POST request.
- Retrieve a list of all tasks with a GET request.
- Update the status of tasks (completed/pending) with a PUT request.
- Delete tasks with a DELETE request.

## Prerequisites

Ensure you have Docker and Rust installed on your system. Rust can be downloaded and installed from the [official Rust website](https://www.rust-lang.org/tools/install). Docker can be obtained from the [Docker website](https://www.docker.com/get-started).

## Installation

1. Clone the repository to your local machine:

```sh
git clone https://github.com/marcelofeitoza/rust-axum-todolist
```

2. Navigate to the project directory:

```sh
cd rust-axum-todolist
```

3. Start the PostgreSQL database and pgAdmin using Docker Compose:

```sh
docker-compose up -d
```

## Running the Application

To run the application, execute:

```sh
cargo run
```

The server will start on `http://localhost:5500`. You can interact with the API using tools like `curl` or API clients such as Postman or Insomnia.

## Using the Application

Access the application endpoints as follows:

- **Create Task**: `POST http://localhost:5500/tasks`
- **List Tasks**: `GET http://localhost:5500/tasks`
- **Toggle Task Status**: `PUT http://localhost:5500/tasks/{id}`
- **Delete Task**: `DELETE http://localhost:5500/tasks/{id}`

pgAdmin is available at `http://localhost:5050` for database management.

## Environment Variables

Set the following environment variables for your application:

- `DATABASE_URL`: The database connection string for your PostgreSQL instance.

## Docker Services

The `docker-compose.yml` file contains the following services:

- **postgres**: The PostgreSQL database server.
- **pgAdmin**: A web-based administration tool for PostgreSQL.
