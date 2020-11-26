# Configuration

LibreCTF is configured using the language [Dhall][1]. It introduces many
type-safety-related benefits. 

There's 2 sets of configuration:

- the deployment config, which is loaded once when the platform is started. The
  platform must be restarted in order to reload this config
- the contest config, which can be freely hot-reloaded as the platform is
  running

TODO: docker environment variables

## Deployment Config

The schema for the deployment config can be found in `./schema/deploy.dhall`.

## Contest Config

The schema for the deployment config can be found in `./schema/contest.dhall`.

[1]: https://dhall-lang.org/
