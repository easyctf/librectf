watch:
    cargo watch --ignore "frontend/*" -x 'build'

check:
    cargo watch --ignore "frontend/*" -x 'check'

run:
    cargo watch --ignore "frontend/*" -x 'run -- web'
