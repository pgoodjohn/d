# D - The Development Helper

`d` provides a standardized way to run development enviroments across diffrent projects.

## Usage

You can use `d` to start your development environments with just a few characters:

```sh
$ d start
$ d s
```

`d` will look for a `d.toml` in your current working directory for instructions for what command to run.

You can create a new `d.toml`, or reset your existing one, by running:

```
$ d init
```

## Configuration

`d` only supports the `start` command as of now. The structure of `d.toml` is as follows:

```toml
[start]
command = ""
arguments = ""
```

## Using with oh-my-zsh

[Oh My ZSH](https://github.com/ohmyzsh/ohmyzsh) already offers the single character `d` as a command to list the directories history. To use `d`, the development helper, you must change the declaration of this function in your [directories.zsh](https://github.com/ohmyzsh/ohmyzsh) to something else other than `d()`.