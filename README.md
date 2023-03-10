# spin

![](https://github.com/serval/spin/actions/workflows/main.yml/badge.svg)

`spin` is a basic tool for running multiple copies of a command simultaneously and prefixing their
standard output / standard error streams with the process ID of each copy.

For example, to spin up 2 copies of `ping 1.1.1.1`:

```shell
$ spin 2 ping 1.1.1.1
[13673] PING 1.1.1.1 (1.1.1.1): 56 data bytes
[13673] 64 bytes from 1.1.1.1: icmp_seq=0 ttl=54 time=25.024 ms
[13694] PING 1.1.1.1 (1.1.1.1): 56 data bytes
[13694] 64 bytes from 1.1.1.1: icmp_seq=0 ttl=54 time=25.483 ms
[13673] 64 bytes from 1.1.1.1: icmp_seq=1 ttl=54 time=32.584 ms
...
```

When you terminate `spin`, all of the child processes will terminate as well.

## Installation

This project uses [just](https://github.com/casey/just) (`brew install just`) for development workflows and automation. Run `just` with no arguments to see a list of available commands.

Once you have installed `just`, you can install `spin` with:

```shell
just install
```

Similarly, you can uninstall it with:

```shell
just uninstall
```
