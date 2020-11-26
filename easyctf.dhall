let Competition = ./schema/competition.dhall in
let Deploy = ./schema/deploy.dhall in

let competition = Competition.Config :: {
  startDate = "2021-01-01T00:00:00+00:00",
  endDate = "2021-01-08T00:00:00+00:00",
  mailer = Competition.Mailer.Smtp Competition.Smtp :: {
    host = "easyctf.com",
  },
} in

let deploy = Deploy.Config :: {
  adminEmail = "team@easyctf.com",
  environment = Deploy.Environment.Development,
  secretKey = "asdf",
} in

{ competition, deploy }
