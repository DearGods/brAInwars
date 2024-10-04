DROP INDEX nonces_wallet_address;
CREATE UNIQUE INDEX nonces_wallet_address ON nonces (wallet_address);