let Mailgun = {
  Type = {
    apiKey : Text,
  },
  default = {=},
} in

let Smtp = {
  Type = {
    host : Text,
  },
  default = {=},
} in

let Mailer = <
  Mailgun : Mailgun.Type |
  Smtp : Smtp.Type
> in

let Config = {
  Type = {
    maxTeamSize : Optional Natural,
    startDate : Text,
    endDate : Text,
    mailer : Mailer,
  },
  default = {
    maxTeamSize = None Natural,
  },
} in

{ Config, Mailgun, Smtp, Mailer }
