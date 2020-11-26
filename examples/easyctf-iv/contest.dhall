let Contest = ./schema/competition.dhall in

Contest.Config :: {
  name = "EasyCTF IV",
  startDate = "2021-01-01T00:00:00+00:00",
  endDate = "2021-01-08T00:00:00+00:00",
  mailer = Competition.Mailer.Smtp Competition.Smtp :: {
    host = "easyctf.com",
  },
}
