# Redcar [![Crates.io][crates-badge]][crates-url] [![Build Status][build-badge]][build-url] [![DockerImage Build Status][docker-ci-badge]][docker-ci-url] [![license][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/redcar.svg
[crates-url]: https://crates.io/crates/redcar
[build-badge]: https://github.com/redcar-io/redcar/actions/workflows/redcar.yml/badge.svg
[build-url]: https://github.com/redcar-io/redcar/actions
[docker-ci-badge]: https://github.com/redcar-io/redcar/actions/workflows/docker-image.yml/badge.svg
[docker-ci-url]: https://github.com/redcar-io/redcar/actions
[license-badge]: https://img.shields.io/badge/license-Apache2-orange.svg?style=flat
[license-url]: https://github.com/redcar-io/redcar/main/LICENSE
[cloudevents_io]: https://cloudevents.io
[serverless_wg]: https://github.com/cncf/wg-serverless
[data-hub]: https://en.wikipedia.org/wiki/Data_hub
[pr-faq]: https://github.com/redcar-io/redcar/blob/master/docs/PRFAQ.md
[redcar]: https://github.com/redcar-io/redcar/blob/master/docs/Redcar.md

A real-time event-oriented data-hub. It has the capabilities of event triggering, aggregation, routing, and storage. 
Provide a high-performance event hosting solution to help users solve the asynchronous problem between services (or functions). 
Use the event-driven architecture(EDA) model to improve the energy efficiency of modern clouds. It is:

* **Fast**: benchmarked 15000s of writes/s per instance and 20000s of reads/s.
* **Universal**: the front end uses gRPC to provide services.
* **Reliable**: high-performance memory storage engine and persistent back-end data warehouse.
* **Secure**: optional SSL client cert authentication.

> If you want a more comprehensive understanding of Redcar, please go to [Redcar][redcar].

## Contents

- [Overview](#overview)
- [Install](#install)
  - [Building](#building)
- [Usage](#usage)
- [Clients](#clients)
- [Maintainers](#maintainers)
- [Contributing](#contributing)

## Overview

Event-driven computing system architecture has become the main governance solution and technology evolution 
direction of various cloud vendors and cloud-native technology systems. To this end we initiated the `Redcar`
project, focusing on:

* Event cast, including multiple modes such as unicast, multicast, multicast and broadcast.
* Exchange, provide routing capabilities between producers and consumers through the `event bus`.
* Persistence, the ability to record and monitor the entire life cycle of events.
* Replay, provides the ability to repeat historical events based on the persistent storage of events.


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

This will generate a binary called `/usr/local/bin/redcar` and `redcar-ctl`.

_NOTE_: you need rust 1.55+. Please check your installation with

```
$ rustc --version
```

## Usage

First start a redcar machine:

```sh
$ redcar -a 127.0.0.1:8519 -o stdout
```

Next, let's set a single key, and then retrieve it:

```sh
$ redcar-ctl -e http://127.0.0.1:8519 "foo bar" put 
$ redcar-ctl -e http://127.0.0.1:8519 "foo" range
```

## Clients

These are the clients for Redcar:

- [Go](https://github.com/redcar-io/client-go) (The most stable and widely used)
- [Rust](https://github.com/redcar-io/redcar/tree/master/client)

If you want to try the Go client, see [Go Client](https://github.com/redcar-io/client-go/).

## Maintainers

[@Redcar](https://github.com/redcar-io).

## Contributing

Feel free to dive in! [Open an issue](https://github.com/redcar-io/redcar/issues/new) or submit PRs.

Redcar follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.
