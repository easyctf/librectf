watch:
    localenv .env cargo watch --ignore "frontend/*" -x 'build'

check:
    localenv .env cargo watch --ignore "frontend/*" -x 'check'

run-admin:
    localenv .env cargo watch --ignore "frontend/*" -x 'run -p openctf-admin'

run-fs:
    localenv .env cargo watch --ignore "frontend/*" -x 'run -p filestore'

run-web:
    localenv .env cargo watch --ignore "frontend/*" -x 'run -p openctf-web'
