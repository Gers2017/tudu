# Tudu

Command line tool to keep track of your todos

### Schema

```toml
[Commit to master]!!!!
- fix typo
- read the docs
[Task]
    - step 1
    - step 2
```
- The text surrounded by square brackets is the title of the task. It should be short and descriptive
- The number of exclamation marks is the priority of the task. A task can have zero exclamation marks
- The indentation will be lost once the program adds a new task

## Usage

Available commands
- get
- add
- rm

### Get subcommands

```bash
tudu get -A | tudu get all
```
```bash
tudu get -P | tudu get primary
```
```bash
tudu get -T 'task' | tudu get title 'task'
```

### Rm subcommands

```bash
tudu rm -A | tudu rm all
```
```bash
tudu rm -P | tudu rm primary
```
```bash
tudu rm -T 'task' | tudu rm title 'task'
```

