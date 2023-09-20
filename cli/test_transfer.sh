RUST_LOG=trace cargo run -- \
    --api-key "$API_KEY" \
    --partner-code "$PARTNER_CODE" \
    --channel "WEB" \
    transfer \
    --bankacc "$BANK_ACC" \
    --bank siam-commercial \
    --accname "$BANK_NAME" \
    --mobileno $MOBILE_NUM \
    --transaction-by "Test developer" \
    --ref1 "6957f82b34aa4e1dab67466fe74804" \
    --amount=10
