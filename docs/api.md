# API

## Patterns

All of the resources protected behind `LoginRequired` requires that the request contains an `Authorization` header. The contents of this header must be `Token`, followed by a space, followed by the exact string returned from `/user/login`. If this header doesn't exist, or the token isn't valid (isn't signed by the server), then a `401 Unauthorized` will be returned. TODO: example

All of the resources protected behind `TeamRequired` requires that the user identified by the `Authorization` token must be on a team. If this is

Any API call may fail with a `500` response, the status code for **Internal Server Error**. Should such a case occur, please file an issue detailing your specific problem.

### GET `/`

Dummy endpoint.

On success:

- **Status:** `200`
- **Returns:** A string.

### GET `/scoreboard`

Returns the most recently updated version of the scoreboard.

On success:

- **Status:** `200`
- **Returns:** An array of JSON objects with the keys:
  - **name:** The name of the team in the scoreboard entry.
  - **score:** The score of the team.

### GET `/chal/list`

This resource is protected behind `LoginRequired` and `TeamRequired`.

On success:

- **Status:** `200`
- **Returns:** An array of JSON objects with the keys:
  - **title:** The title of the challenge.
  - **value:** The value of the challenge.

TODO: finish this
