let Deploy = ./schema/deploy.dhall in

Deploy.Config :: {
  adminEmail = "team@easyctf.com",
  environment = Deploy.Environment.Development,
  secretKey = "asdf",
}
