**crossterm > command**

# Module: command

## Contents

**Traits**

- [`Command`](#command) - An interface for a command that performs an action on the terminal.
- [`ExecutableCommand`](#executablecommand) - An interface for types that can directly execute commands.
- [`QueueableCommand`](#queueablecommand) - An interface for types that can queue commands for further execution.
- [`SynchronizedUpdate`](#synchronizedupdate) - An interface for types that support synchronized updates.

---

## crossterm::command::Command

*Trait*

An interface for a command that performs an action on the terminal.

Crossterm provides a set of commands,
and there is no immediate reason to implement a command yourself.
In order to understand how to use and execute commands,
it is recommended that you take a look at [Command API](./index.html#command-api) chapter.

**Methods:**

- `write_ansi`: Write an ANSI representation of this command to the given writer.



## crossterm::command::ExecutableCommand

*Trait*

An interface for types that can directly execute commands.

**Methods:**

- `execute`: Executes the given command directly.



## crossterm::command::QueueableCommand

*Trait*

An interface for types that can queue commands for further execution.

**Methods:**

- `queue`: Queues the given command for further execution.



## crossterm::command::SynchronizedUpdate

*Trait*

An interface for types that support synchronized updates.

**Methods:**

- `sync_update`: Performs a set of actions against the given type.



