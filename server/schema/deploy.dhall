let Environment = < Development | Production > in

let Config = {
  Type = {
    adminEmail : Text,
    secretKey : Text,
    environment : Environment,
    sentryDsn : Optional Text,
  },
  default = {
    environment = Environment.Production,
    sentryDsn = None Text,
  },
} in

{ Environment, Config }
