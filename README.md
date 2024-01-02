
# CLI TODO App

  

## Overview

  

The CLI TODO App is a command-line task management tool built in Rust. It provides a simple and interactive way to manage tasks, allowing users to add, update, delete, and view tasks across three statuses: Backlog, To Do, and Done. The app also features data persistence, saving tasks in a JSON file so that they are retained between sessions.

  

## Features

  

-  **Add Tasks:** Create new tasks and assign them a status (Backlog, To Do, or Done).

-  **Update Tasks:** Change the status of existing tasks.

-  **Delete Tasks:** Remove tasks that are no longer needed.

-  **Display Tasks:** View all tasks in a neatly formatted table, organized by their status.

-  **Data Persistence:** Tasks are saved in a JSON file (`tasks.json`), ensuring that your task list persists between application runs.

  

## How to Use

  

1.  **Start the App:** Run the app in your command line or terminal. You'll be presented with a menu of options.

  

2.  **Menu Options:**

-  `1: Add task` - Enter a task name and select a status to add a new task.

-  `2: Update task` - Choose an existing task to update its status.

-  `3: Display tasks` - View all tasks and their statuses in a table.

-  `4: Delete task` - Select a task to delete it from the list.

-  `5: Exit` - Exit the application. All changes are saved automatically.

  

3.  **Interacting:** Follow the on-screen prompts to interact with the app. Enter the number corresponding to your choice and provide any additional required information (like task names or statuses).

  

## Installation and Setup

  

- Ensure you have Rust installed on your system.

- Clone the repository and navigate to the project directory.

- Build and run the application using Cargo:

```bash
$ cargo build
$ cargo run
```

  

## Data Storage

  

- The app uses a file named tasks.json for persisting task data. This file will be automatically created in the project directory upon running the application, and it will be updated as you add, update, or delete tasks.

  

## Dependencies

* `serde`: For JSON serialization and deserialization.

* `comfy-table`: For displaying tasks in a table format.
