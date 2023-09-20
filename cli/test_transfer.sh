RUST_LOG=trace cargo run -- \
    --api-key "***REMOVED***" \
    --partner-code "CRS" \
    --channel "WEB" \
    transfer \
    --bankacc "0652078409" \
    --bank kasikorn \
    --accname "Manop Tangngam" \
    --mobileno 0805933181 \
    --transaction-by "Test developer" \
    --ref1 "123456789" \
    --amount=100.5
