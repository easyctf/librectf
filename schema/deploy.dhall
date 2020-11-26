let Environment = < Development | Production > in

let Config = {
  Type = {
    adminEmail : Text,
    secretKey : Text,
    environment : Environment,
  },
  default = {
    environment = Environment.Production,
  },
} in

{ Environment, Config }
