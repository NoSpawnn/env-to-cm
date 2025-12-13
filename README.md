# env-to-cm

- Convert `.env` files directly to kubernetes `ConfigMap`s!

## Why?

- Because, yes, you _can_ just do:

    ```sh
    kubectl create configmap myconfigmap --from-env-file=.env --dry-run=client -o yaml
    ```

    - But this doesn't preserve, whitespace or comments, and the output will be in a random order

- And you can also generate `ConfigMap`s directly on a cluster from a `.env` file, but this requires a live cluster 
- Instead, this tool translates these files whilst maintaining the order of values, whitespace between them, and any comments, with no dependency on a cluster

## Status/Todos

- [x] Basic translation of `.env` to `ConfigMap` yaml
- [ ] Dead text preservation
    - [ ] Preserve whitespace
    - [ ] Preserve comments
    - [ ] Any combination of the above
