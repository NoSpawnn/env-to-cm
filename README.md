# env-to-cm

- Convert `.env` files directly to kubernetes `ConfigMap`s!

## Why?

- Because, yes, you _can_ just do:

    ```sh
    kubectl create configmap myconfigmap --from-env-file=.env --dry-run=client -o yaml
    ```

    - But this doesn't preserve, whitespace or comments, and the output will be in a random order

- You _can_ also generate `ConfigMap`s directly on a cluster from a `.env` file, but this requires a live cluster 
- Instead, this tool translates these files whilst maintaining the order of values, whitespace between them, and any comments, with no dependency on an active cluster
- My use case for this is to convert [Docker Compose](https://docs.docker.com/compose/) stacks using environment files to Podman [Kube quadlets](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html#kube-units-kube), which dont support the `EnvironmentFile=` key like [Container units](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html#container-units-container)
    - It's a great companion to [containers/podlet](https://github.com/containers/podlet)

## Status/Todos

- [x] Basic translation of `.env` to `ConfigMap` yaml
- [x] Dead text preservation
    - [x] Preserve whitespace
    - [x] Preserve comments
    - [x] Any combination of the above

# Usage

todo!()

# Dev usage/development

1. Clone this repo

```sh
git clone https://github.com/NoSpawnn/env-to-cm.git
```

2. Run

```sh
cargo r -- --help
```
