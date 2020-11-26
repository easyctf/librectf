# Configuration

LibreCTF is configured using the language [Dhall][1]. It introduces many
type-safety-related benefits. Before running, make sure the config files
`deploy.dhall` and `contest.dhall` live in the same directory as
`docker-compose.yml`. The docker compose config will mount these dhall files
into the volume when it starts the application.

There's 2 sets of configuration:

- the deployment config, which is loaded once when the platform is started. The
  platform must be restarted in order to reload this config
- the contest config, which can be freely hot-reloaded as the platform is
  running

## Deployment Config

The schema for the deployment config can be found in `./schema/deploy.dhall`.

## Contest Config

The schema for the deployment config can be found in `./schema/contest.dhall`.

[1]: https://dhall-lang.org/
