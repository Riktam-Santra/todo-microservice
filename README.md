# About
A simple microservice for todos written using Rust and Postgres.

# **MADE IN HURRY!!**
I just speed ran through this and everything is **unstable** since I have only used unwrap() everywhere, I'll fix this soon.

# Installation

1. Clone the repository using `git clone https://github.com/Riktam-Santra/todo-microservice`.
2. Copy `.env.example` file into `.env` file and set the DATABASE_URL variable to your postgres database url.
3. Make sure your postgres is running and `sqlx migrate run` to setup the database and tables. It creates a database by the name `todos` and a table by the name `todos`. You can change the SQL for this in `migrations` folder.
4. Finally run the server using cargo run

# Endpoints
## POST /todos
Adds a todo in the list. Your request must have the following body:
```json
[
    {
        "title" : "string",
        "task_completed": "boolean",
    }
]
```
## GET /todos
Gets all todos in the following JSON schema:
```json
[
    {
        "id" : "uuid",
        "title" : "string",
        "task_completed": "boolean",
        "date_created": [
            "year",
            "date"
        ]
    }
]
```
## PUT /todos/{uuid}
Updates a todo with the corresponding `uuid`. Your body can have a optional JSON with atleast one of the fields.
```json
    {
        "title" : "string",
        "task_completed": "boolean",
    }
```
## DELETE /todos/{uuid}
Deletes a todo.

## POST /todos/{uuid}/complete
Marks a todo with id `uuid` as completed.

## POST /todos/{uuid}/revert
Marks a todo with id `uuid` as incompleted.