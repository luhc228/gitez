# gitez

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/gitez.svg
[crates-url]: https://crates.io/crates/gitez

Gitez(ez means easy) is a CLI that simplifies managing your Git repositories.

## Installation

Cargo

```shell
cargo install gitez
```

<details>

<summary>Macos and Linux</summary>

Coming soon...

</details>

<details>

<summary>Windows</summary>

Coming soon...

</details>

## Features

### Clone Repository

gitez provides a simple way to manage your git repositories.

```md
$BASE
|- github.com
|  `- group-name
|     `- project-name
`- gitlab.com
   `- group-name
      `- project-name
```

First, you need to set the base directory where you want to store your repositories.

```shell
gitez set-base-dir <your-base-dir> # gitez set-base-dir ~/dev
```

Second, clone a git repository.

```shell
gitez clone <repository-url> # gitez clone git@github.com:luhc228/gitez.git
```

Then, you will get the following directory structure.

```md
$BASE
`- github.com
   `- luhc228
      `- gitez
```

### Manage Git User Config

Do you ever have one more git user configs( for example one is for open source(GitHub) and another is for Work(GitLab))? And you use different username and email for your commits at work or for your open source repositories by mistake?

gitez helps you manage and switch your git user configs easier.

First, you need to add your git user configs.

```shell
gitez user-config add

# see the list of your git user configs you added
gitez user-config list
```

Second, you can set a directory that uses one of yor git user configs.(powered by [gitdir config](https://git-scm.com/docs/git-config#Documentation/git-config.txt-codegitdircode))

```shell
gitez user-config add-include <user-config-name> # gitez user-config add-include
```

```md
$BASE
`- github.com  # use github user config(name<a@example.com>)
`- gitlab.com  # use gitlab user config(name<b@example.com>)
```

Also, you can apply one of your git user configs to the current directory.

```shell
gitez user-config apply <user-config-name> # gitez user-config apply
```

### Generate SSH Key

You can generate an SSH key for your git repositories by the following command:

```shell
gitez ssh-keygen
```

It will also add the SSH private key to the `~/.ssh/config` so that you can manage different SSH private keys for different git repositories.
