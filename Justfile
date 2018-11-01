watch:
    localenv .env cargo watch --ignore "frontend/*" -x 'build'

check:
    localenv .env cargo watch --ignore "frontend/*" -x 'check'
