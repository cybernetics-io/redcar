# Redcar 

[![Crates.io][crates-badge]][crates-url]
[![Build Status][build-badge]][build-url]
[![DockerImage Build Status][docker-ci-badge]][docker-ci-url]
[![license][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/redcar.svg
[crates-url]: https://crates.io/crates/redcar
[build-badge]: https://github.com/redcar-io/redcar/actions/workflows/redcar.yml/badge.svg
[build-url]: https://github.com/redcar-io/redcar/actions
[docker-ci-badge]: https://github.com/redcar-io/redcar/actions/workflows/docker-image.yml/badge.svg
[docker-ci-url]: https://github.com/redcar-io/redcar/actions
[license-badge]: https://img.shields.io/badge/license-Apache2-orange.svg?style=flat
[license-url]: https://github.com/redcar-io/redcar/main/LICENSE
[cloudevents_io]:https://cloudevents.io
[serverless_wg]:https://github.com/cncf/wg-serverless

A real-time event-oriented database, inspired by the [cloudevents][cloudevents_io] of the `CNCF's` 
[serverless working group][serverless_wg]. It is:

* **Universal**:
* **Fast**:
* **Reliable**:
* **Secure**:

## Table of Contents

- [Overview](#overview)
- [Install](#install)
- [Usage](#usage)
    - [Generator](#generator)
- [Example](#example)
- [Maintainers](#maintainers)
- [Contributing](#contributing)


## Overview

Event-driven computing system architecture has become the main governance solution and technology evolution 
direction of various cloud vendors and cloud-native technology systems. To this end we initiated the `Redcar`
project, focusing on:

* Event cast, including multiple modes such as unicast, multicast, multicast and broadcast.
* Exchange, :
* Replay, :
* Persistence, :

The vision is to explore and improve cloud event standards together with cloud native. Our goal is to achieve 
an event-driven engine.

## Install
The latest release and setup instructions are available at [GitHub][github-release].

[github-release]: https://github.com/redcar-io/redcar/releases/

### Building

You can build redcar from source:

```sh
$ git clone https://github.com/redcar-io/redcar/
$ cd redcar
$ ./make
$ ./make install
```

This will generate a binary called `./bin/redcar`.

_NOTE_: you need rust 1.55+. Please check your installation with

```
$ rustc --version
```

## Usage

First start a redcar machine:

```sh
$ ./bin/redcar -a 127.0.0.1:8519 -o stdout
```

Next, let's set a single key, and then retrieve it:

```sh
$ ./bin/redcar-ctl -e http://127.0.0.1:8519 "foo bar" put 
$ ./bin/redcar-ctl -e http://127.0.0.1:8519 "foo" range
```

### Generator

To use the generator, look at . There is a global executable to run the generator in that package, aliased as .

## Example

To see how the specification has been applied, see the [example](example/).

## Maintainers

[@Redcar](https://github.com/redcar-io).

## Contributing

Feel free to dive in! [Open an issue](https://github.com/redcar-io/redcar/issues/new) or submit PRs.

Standard Readme follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.
