# Tudu

Command line tool to keep track of your tasks/todos

![get cmd in terminal](tudu.gif)

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
- The number of exclamation marks is the priority of the task. Tasks can have zero exclamation marks
- The indentation is optional, just keep in mind that it'll be lost if the cli adds a new todo

## Usage

### Tudu cli

```sh
tudu [get, rm, add] [--help] # subcomamnds and flags
```

### Get SubCommand

```sh
tudu get [-a, --all, -p, --primary, -t, --title, --help]
```

### Rm SubCommand

```sh
tudu rm [-a, --all, -p, --primary, -t, --title, --help]
```

### Add SubCommand

```sh
tudu add [--help]
```
