# Crib

![](https://private-user-images.githubusercontent.com/10061744/418248640-7a51fd46-5c4f-48a9-b627-b9fe7558ef20.gif?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NDA4MDgxMDYsIm5iZiI6MTc0MDgwNzgwNiwicGF0aCI6Ii8xMDA2MTc0NC80MTgyNDg2NDAtN2E1MWZkNDYtNWM0Zi00OGE5LWI2MjctYjlmZTc1NThlZjIwLmdpZj9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAzMDElMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMzAxVDA1NDMyNlomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTcyOTAyNjg1YTk0MjY1M2E5MDgzYjBiNTI5Y2E0MDM4YWJjMWM0Yzk3YjNjNTZmMDhhYjZiNTljZTU4ZjkxMjkmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.qK121ZaHhNBfAEEGakHmRMRXhM_PqHBr7oNuAQROVPg)

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
