let Config = {
  Type = {
    title : Text,
    author : Text,
    category : Text,
    value : Natural,
    hint : Text,
    description : Text,
    autogen : Bool,
    files : List Text,
  },
  default = {
    autogen = True,
    files = [] : List Text,
  },
} in

{ Config }
