doc:
    cargo watch -x 'doc --no-deps --document-private-items'

run:
    RUST_LOG="frontend=info" cargo watch -x 'run -- run --bind-addr 0.0.0.0:3000 --database-uri=sqlite:///home/michael/Projects/openctf/test.db --secret-key asdfasdfasdfasdfasdfasdfasdfasdf'
