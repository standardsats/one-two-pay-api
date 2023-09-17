RUST_LOG=trace cargo run -- \
    --api-key "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9eyJwYXJ0bmVyX2NvZGUiOiJUQ1UiLCJ1dWlkIjoiMTBiN2M2NzgtNDM3ZC00MTI2LWFhZTctYjNmOThjYzEwZjhjIn0.Oyco4W9vwpJYBBh671Fk3UzxPgDFTenD8m3h_l76VgU" \
    --partner-code "ABC" \
    --channel "WEB" \
    inquery \
    --ref1 "123456789"
