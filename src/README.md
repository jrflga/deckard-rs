# Replicant-RS

Replicant-RS is a robust CLI tool designed for copying databases between environments. It offers flexibility in data selection, scheduling, and ensures data integrity during the copy process.

## Features

- **Environment Configuration:** Easily configure source and target environments with detailed database credentials.
- **Data Selection:** Support for copying entire databases, specific tables, or filtered rows.
- **Data Filtering:** Utilize SQL-like queries or regex for precise data selection.
- **Scheduling:** Set up cron jobs for automated data copying tasks.
- **Data Transformation:** Apply transformations to data before copying to the target environment.
- **Logging and Monitoring:** Comprehensive logging for effective monitoring and troubleshooting.
- **Dry Run Mode:** Simulate copy operations to preview outcomes without actual execution.
- **Concurrency Control:** Manage concurrent table copies for optimal performance.
- **Integrity Check:** Post-copy data integrity verification to ensure data consistency.
- **Resume Capability:** Resume interrupted copy tasks seamlessly.

## Installation
*Instructions for installing replicant-rs.*

## Usage
### Configure Environments
```bash
replicant-rs config set --env dev --db-type postgres --host localhost --port 5432 --user dev_user --password dev_pass

replicant-rs config set --env prod --db-type mysql --host prod-host --port 3306 --user prod_user --password prod_pass
```

### Copy Operations
- #### Entire Database

```bash
replicant-rs copy --source dev --target prod
```

- #### Specific Tables

```bash
replicant-rs copy --source dev --target prod --tables users,orders
```

- #### Filtered Row Copying

```bash
replicant-rs copy --source dev --target prod --table users --where "created_at > '2023-01-01'"
```

- #### Scheduling with Cron

```bash
replicant-rs schedule --source dev --target prod --cron "0 0 * * *"
```

## Advanced Options
### Data Transformation

```bash
replicant-rs copy --source dev --target prod --transform "UPDATE users SET email = CONCAT('dev_', email)"
```

Dry Run

```bash
replicant-rs copy --source dev --target prod --dry-run
```

Concurrency Control

```bash
replicant-rs copy --source dev --target prod --concurrency 5
```

Integrity Check

```bash
replicant-rs copy --source dev --target prod --integrity-check
```

Resume Capability

```bash
replicant-rs resume --task-id 12345
```

## Data Selection Mechanisms

- Default to entire database copying if no specific table or filter is provided.
- Use --tables for selecting specific tables.
- Use --where for SQL-like row filtering.
- Advanced users can specify custom SQL queries with --query.

## Implementation Notes
- Ensure database-agnostic support for popular systems like MySQL, PostgreSQL, and MongoDB.
- Leverage native database tools for efficiency in dump and restore operations.
- Emphasize robust error handling, validation, and security best practices.