let Environment = < Development | Production > in

let Config = {
  Type = {
    adminEmail : Text,
    secretKey : Text,
    environment : Environment,
    sentryDsn : Optional Text,
    disableEmails : Bool,
  },
  default = {
    environment = Environment.Production,
    sentryDsn = None Text,
    disableEmails = False,
  },
} in

{ Environment, Config }
