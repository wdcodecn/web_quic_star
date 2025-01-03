-- This file should undo anything in `up.sql`




CREATE TABLE "wallet_token_recoards"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"wallet_id" INT8 NOT NULL,
	"token_id" INT8 NOT NULL,
	"token_address" TEXT NOT NULL,
	"token_amount" INT8 NOT NULL,
	"token_usd" INT8 NOT NULL,
	"is_buy" TEXT NOT NULL,
	"price" INT8 NOT NULL
);

DROP TABLE IF EXISTS "block_sync";
DROP TABLE IF EXISTS "wallet_tokens";
DROP TABLE IF EXISTS "token_pairs";
DROP TABLE IF EXISTS "wallet_transactions";
DROP TABLE IF EXISTS "wallet_token_trades";
DROP TABLE IF EXISTS "wallets";
