# Crib

![](https://i.imgur.com/saTPUrl.gif)

Create and view your own custom hotkey cheatsheet in the terminal

## Motivation
I had trouble remembering the various hotkeys that I sometimes use. It got
annoying to look them up so I resorted to writing them down on a paper
cheatsheet. Then, I thought: maybe there's a tool that does this better. I
didn't find one I liked so I built crib.

With crib, I can list:
* Hotkeys that I occasionally forget or am new to
* Custom key combinations that I defined for my own workflow
It is best used as a popup cheatsheet.


## Install
```
cargo install --git https://github.com/noelzubin/crib.git
```

## Build from source
```
cargo build --release
```

## Usage
To perform filtering on section headings, append `:` to the search term. For
example, to search for all sections with vscode in the heading, type `vscode:`.

To scroll use up/down arrow keys.

## Configuration syntax
``` yaml
- name: yazi
  bindings:
  - name: Toggle hidden files
    key: "."
  - name: Run a shell command
    key: ":"
  children:
    - name: find files
      bindings:
      - name: zoxide
        key: z
      - name: fzf
        key: Z
      - name: fd
        key: s
    - name: misc
      bindings:
      - name: help
        key: F1
      - name: quit without writing CWD
        key: Q
- name: vscode
  children:
    - name: lsp
      bindings:
        - name: format document
          key: =
        - name: focus explorer
          key: ge
```

#### Extra options 
``` sh
crib # run tui 

# crib <query>
crib vscode # filter app by query.

crib --print # print to the console instead of running the tui. 
```


# Similar tools
Inspired by:
* [showkeys](https://github.com/adamharmansky/showkeys)
* [keyb](https://github.com/kencx/keyb)
