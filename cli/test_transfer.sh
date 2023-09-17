RUST_LOG=trace cargo run -- \
    --api-key "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9eyJwYXJ0bmVyX2NvZGUiOiJUQ1UiLCJ1dWlkIjoiMTBiN2M2NzgtNDM3ZC00MTI2LWFhZTctYjNmOThjYzEwZjhjIn0.Oyco4W9vwpJYBBh671Fk3UzxPgDFTenD8m3h_l76VgU" \
    --partner-code "ABC" \
    --channel "WEB" \
    transfer \
    --bankacc "0652078409" \
    --bank kasikorn \
    --accname "Manop Tangngam" \
    --mobileno 0805933181 \
    --transaction-by "Test developer" \
    --ref1 "123456789" \
    --amount=100.5
