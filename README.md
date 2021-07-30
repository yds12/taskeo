To-do or task management application for the terminal.

# Intended API

To display tasks:

    $ keo
    <displays tasks>

It might be convenient to call `keo` on your `.bashrc` or equivalent to show
your to-dos automatically when you open a new terminal.

To add a task:

    $ keo ** buy mangos

Use 0 to 4 `*` to define the priority level of the task. This will be used by
`taskeo` to decide the order and color of the task in the list.

Remove a task:

    $ keo <N>

where `<N>` is a the task number, which is shown before the task description
in the task list.

Inspired by the terminal to-do script used by
[Jon Gjengset](https://www.youtube.com/c/JonGjengset/videos) in one of his
videos.

