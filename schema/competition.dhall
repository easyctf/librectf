let Config = {
  Type = {
    maxTeamSize : Optional Natural,
    startDate : Text,
    endDate : Text,
  },
  default = {
    maxTeamSize = None Natural,
  },
} in

{ Config }
