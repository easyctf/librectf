watch:
    cargo watch --ignore "frontend/*" -x 'build'

check:
    cargo watch --ignore "frontend/*" -x 'check'

run-fs:
    cargo watch --ignore "frontend/*" -x 'run -p filestore'

run-web:
    cargo watch --ignore "frontend/*" -x 'run -p openctf-web'
