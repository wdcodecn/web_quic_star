-- Your SQL goes here




DROP TABLE IF EXISTS "wallet_token_recoards";
CREATE TABLE "block_sync"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"block_number" INT8 NOT NULL
);

CREATE TABLE "wallet_tokens"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"wallet_id" INT8 NOT NULL,
	"wallet_address" TEXT NOT NULL,
	"token_name" TEXT NOT NULL,
	"token_address" TEXT NOT NULL,
	"token_balance" INT8 NOT NULL,
	"token_decimals" INT8 NOT NULL
);

CREATE TABLE "token_pairs"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"token0_address" TEXT NOT NULL,
	"token0_name" TEXT NOT NULL,
	"token0_decimals" INT8 NOT NULL,
	"token1_address" TEXT NOT NULL,
	"token1_name" TEXT NOT NULL,
	"token1_decimals" INT8 NOT NULL,
	"pair_address" TEXT NOT NULL,
	"pair_name" TEXT NOT NULL
);

CREATE TABLE "wallet_transactions"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"wallet_id" INT8 NOT NULL,
	"wallet_address" TEXT NOT NULL,
	"transaction_to" TEXT NOT NULL,
	"transaction_nonce" INT8 NOT NULL,
	"transaction_hash" TEXT NOT NULL,
	"transaction_value" INT8 NOT NULL,
	"transaction_block_number" INT8 NOT NULL,
	"transaction_block_hash" TEXT NOT NULL,
	"transaction_block_timestamp" TIMESTAMPTZ NOT NULL,
	"transaction_block_index" INT8 NOT NULL
);

CREATE TABLE "wallet_token_trades"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
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

CREATE TABLE "wallets"(
	"id"          SERIAL8     NOT NULL PRIMARY KEY,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL,
	"chain_id" INT8 NOT NULL,
	"wallet_address" TEXT NOT NULL,
	"block_number" INT8 NOT NULL,
	"is_pending" BOOL NOT NULL
);

