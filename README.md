# git-sift
A command line tool to help you `checkout` when you have too many branches to remember.

`sift` is a command line tool that gives you a TUI to fuzzy-search your git branches, listing both local and remote branches.
Optionally, you can append a command line parameter which will be used as the initial query.

Checking out local branches behaves exactly as you'd expect. However, the behaviour of checking out a remote ref depends on the state of your repo:
Checking out a remote-only ref sets up a local tracking branch and checks it out.
Checking out a remote ref when there is a local tracking branch has two possible outcomes:
1. The remote ref and local branch point to the same commit: the local branch is checked out.
2. The remote ref and local branch point to different commits: the remote ref is checked out in the detached HEAD state.

## Build Requirements:
To build/install from source, you'll need:
- Rust and Cargo available

## Building/Installation
To build:
```
$ cargo build -r
```
To install:
Either copy the resulting binary from `target/release/sift` into your favorite binary directory (you'll need to make
sure you maintain execute permissions), or run:
```
$ cargo install --path .
```

Alternatively, a Linux (x64) binary release is provided.

## Why does this exist?
To solve a problem I run into at work a lot. To be honest, something like:
```
git checkout "$(git branch --all --color=never | grep -v HEAD | sed 's#remotes/[^/]*/##' | sort -u | fzf)"
```
does the trick just as well. But, I've been wanting to learn me a bit of Rust and this seemed like a good way to do it so... here we are!
