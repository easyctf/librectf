let Environment = < Development | Production > in

let Mysql = {
  host : Text,
  port : Natural,
  user : Text,
  pass : Text,
  db : Text,
} in

let Sqlite = {
  path : Text,
} in

let Database = < Mysql : Mysql | Sqlite : Sqlite > in

let Config = {
  Type = {
    adminEmail : Text,
    secretKey : Text,
    environment : Environment,
    sentryDsn : Optional Text,
    disableEmails : Bool,
    database : Database,
  },
  default = {
    environment = Environment.Production,
    sentryDsn = None Text,
    disableEmails = False,
  },
} in

{ Environment, Config, Database, Sqlite, Mysql }
