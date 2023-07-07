# ope

a CLI tool for reminding yourself of all your flaws

## usage

call it with a quick note to save the note:

```
$ ope i "hacked in a quick fix to get the frontend running"
```

view the notes:

```
$ ope look
2023-07-06T23:32:07.414061461Z [HACK] hacked in a quick fix to get the frontend running
```

call it with 'will' to save the note as a todo instead:

```
$ ope will "add support for easier API calls"
$ ope will "set up libinput-gestures"
$ ope look
FIX THIS:
2023-07-06T23:32:07.414061461Z [HACK] hacked in a quick fix to get the frontend running

DO THIS:
2023-07-06T23:35:07.414061461Z [TODO] add support for easier API calls
2023-07-06T23:35:07.414061461Z [TODO] set up libinput-gestures
```

## installation

clone and build. packaging is WIP

