RUST_LOG=trace cargo run -- \
    --api-key "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJwYXJ0bmVyX2NvZGUiOiJDUlMiLCJ1dWlkIjoiMjNhMzI1YWEtMGZkOC00MTcwLTlhNTktOTg3YTMyNzQyYWZjIn0.AiNS7XABAiDU4Somtq4KaofhOc7-Z3WbPVYsVeuMRgo" \
    --partner-code "CRS" \
    --channel "WEB" \
    transfer \
    --bankacc "6572277888" \
    --bank siam-commercial \
    --accname "Anton Gushcha" \
    --mobileno 0655793938 \
    --transaction-by "Test developer" \
    --ref1 "6957f82b34aa4e1dab67466fe74804" \
    --amount=10
