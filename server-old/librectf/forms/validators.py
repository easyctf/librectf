from wtforms.validators import Length

UsernameLengthValidator = Length(
    3, 16, message="Usernames must be between 3 to 16 characters long."
)
TeamLengthValidator = Length(
    3, 32, message="Usernames must be between 3 to 32 characters long."
)
