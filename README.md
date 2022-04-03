# Tudu

Command line tool to keep track of your tasks/todos

[get cmd in terminal](tudu.gif)

### Schema

```
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

```
tudu get -A | tudu get all
```
```
tudu get -P | tudu get primary
```
```
tudu get -T 'task' | tudu get title 'task'
```

### Rm subcommands

```
tudu rm -A | tudu rm all
```
```
tudu rm -P | tudu rm primary
```
```
tudu rm -T 'task' | tudu rm title 'task'
```

### Help flag

The help flag is available for the get and rm command.
Use the `--help` or `-h` flag as the following example

```
tudu rm -A -h | tudu rm all --help
```
```
tudu get title -h | tudu get -A --help
```