# deckard-rs

deckard-rs is a robust CLI tool designed for copying databases between environments. It offers flexibility in data selection, scheduling, and ensures data integrity during the copy process.

## Features

- **Environment Configuration:** Easily configure source and target environments with detailed database credentials.
- **Data Selection:** Support for copying entire databases, specific tables, or filtered rows.
- **Data Filtering:** Utilize SQL-like queries or regex for precise data selection.
- **Scheduling:** Set up cron jobs for automated data copying tasks.
- **Data Transformation:** Apply transformations to data before copying to the target environment.
- **Logging and Monitoring:** Comprehensive logging for effective monitoring and troubleshooting.
- **Colorful Output:** Stylish CLI output powered by the [lipbalm](https://crates.io/crates/lipbalm) library inspired by charm.sh.
- **Dry Run Mode:** Simulate copy operations to preview outcomes without actual execution.
- **Concurrency Control:** Manage concurrent table copies for optimal performance.
- **Integrity Check:** Post-copy data integrity verification to ensure data consistency.
- **Resume Capability:** Resume interrupted copy tasks seamlessly.

## Installation
*Instructions for installing deckard-rs.*

## Usage
### Configure Environments
```bash
deckard config set --env dev --db-type postgres --host localhost --port 5432 --user dev_user --password dev_pass

deckard config set --env prod --db-type mysql --host prod-host --port 3306 --user prod_user --password prod_pass
```

### Copy Operations
- #### Entire Database

```bash
deckard copy --source dev --target prod
```

- #### Specific Tables

```bash
deckard copy --source dev --target prod --tables users,orders
```

- #### Filtered Row Copying

```bash
deckard copy --source dev --target prod --table users --where "created_at > '2023-01-01'"
```

- #### Scheduling with Cron

```bash
deckard schedule --source dev --target prod --cron "0 0 * * *"
```

## Advanced Options
### Data Transformation

```bash
deckard copy --source dev --target prod --transform "UPDATE users SET email = CONCAT('dev_', email)"
```

Dry Run

```bash
deckard copy --source dev --target prod --dry-run
```

Concurrency Control

```bash
deckard copy --source dev --target prod --concurrency 5
```

Integrity Check

```bash
deckard copy --source dev --target prod --integrity-check
```

Resume Capability

```bash
deckard resume --task-id 12345
```

## Data Selection Mechanisms

- Default to entire database copying if no specific table or filter is provided.
- Use --tables for selecting specific tables.
- Use --where for SQL-like row filtering.
- Advanced users can specify custom SQL queries with --query.

## How tasks works

- **Task Identification:** Each copy operation generate a unique task identifier (task-id). This ID could be a hash or a UUID generated at the beginning of the operation.

- **State Persistence:** During the copy operation, it periodically save the state of the operation to a local file or a lightweight database (like SQLite, if permissible). The state includes details like tables copied, rows processed, timestamps, and any errors encountered.

- **Error Handling:** We ensure robust error handling mechanisms are in place to catch interruptions (like network issues, application errors, etc.). On encountering an error, we save the current state before exiting or crashing.

- **Resuming Logic:** On initiating a resume operation with deckard-rs resume --task-id 12345, the tool reads the persisted state associated with the provided task-id.
The tool then determines where the interruption occurred, such as which table was being copied and the last successfully copied row or batch.
Resume the operation from the point of interruption, ensuring any partially copied data is handled correctly to avoid duplicates or data corruption.

- **Concurrency and Integrity:** For operations involving multiple concurrent tasks, ensure each sub-task (like table copy) maintains its own state. This allows for more granular resumption and better integrity control. With implemented checksums or hash verifications for batches of rows copied to ensure data integrity upon resumption.

- **Cleanup and Finalization:** Once the resume operation successfully completes, we ensure the state is updated to reflect the completion, and any temporary data or states are cleaned up.Optionally, perform a final integrity check to confirm the copied data matches the source post-resumption.

## Additional Examples

- **Postgres to MySQL with integrity check**

```bash
deckard copy --source pg_env --target mysql_env --integrity-check --concurrency 4
```

- **Scheduling frequent syncs**

```bash
deckard schedule --source staging --target prod --cron "*/30 * * * *"
```

- **Resuming a task**

```bash
deckard resume --task-id 67890
```

## Implementation Notes
- Ensure database-agnostic support for popular systems like MySQL, PostgreSQL, and MongoDB.
- Leverage native database tools for efficiency in dump and restore operations.
- Emphasize robust error handling, validation, and security best practices.
