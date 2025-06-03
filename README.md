# Deckard

Deckard is a CLI to copy data between databases and environments. It focuses on flexibility and safe operations while offering a colorful command line experience.

## Features
- Store connection details per environment
- Copy entire databases or selected tables
- Schedule recurring copies
- Resume interrupted tasks


## Installation

```bash
cargo install --path .
```

## Usage

### Help
```bash
$ deckard --help
Database copy tool

Usage: deckard <COMMAND>

Commands:
  config    Manage configuration
  copy      Copy databases
  schedule  Schedule copy task
  resume    Resume a previous task
  help      Print this message or the help of the given subcommand(s)
```

### Configure an environment
```bash
$ deckard config set dev --db-type postgres --host localhost --port 5432 --user dev --password pass
```

### Show stored configuration
```bash
$ deckard config get dev
{
  "db_type": "postgres",
  "host": "localhost",
  "port": 5432,
  "user": "dev",
  "password": "pass"
}
```

### Copy between environments
```bash
$ deckard copy --source dev --target dev
Starting copy task ad9615ec-0c05-433a-8f57-e4ec6b83a47f
From dev to dev
Copy operation completed (simulated)
```

### Schedule a recurring copy
```bash
$ deckard schedule --source dev --target dev --cron "0 0 * * *"
Scheduling copy from dev to dev with cron 0 0 * * *
```

### Resume an interrupted task
```bash
$ deckard resume --task-id 1234
Resuming task 1234
```

## Testing

```bash
cargo test
```
