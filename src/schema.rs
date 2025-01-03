// @generated automatically by Diesel CLI.

diesel::table! {
    block_sync (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        block_number -> Int8,
    }
}

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    groups_permissions (group_id, permission_id) {
        group_id -> Int8,
        permission_id -> Int8,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    token_pairs (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        token0_address -> Text,
        token0_name -> Text,
        token0_decimals -> Int8,
        token1_address -> Text,
        token1_name -> Text,
        token1_decimals -> Int8,
        pair_address -> Text,
        pair_name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        group_id -> Int8,
        tenantry -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    wallet_token_trades (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        wallet_id -> Int8,
        token_id -> Int8,
        token_address -> Text,
        token_amount -> Int8,
        token_usd -> Int8,
        is_buy -> Text,
        price -> Int8,
    }
}

diesel::table! {
    wallet_tokens (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        wallet_id -> Int8,
        wallet_address -> Text,
        token_name -> Text,
        token_address -> Text,
        token_balance -> Int8,
        token_decimals -> Int8,
    }
}

diesel::table! {
    wallet_transactions (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        wallet_id -> Int8,
        wallet_address -> Text,
        transaction_to -> Text,
        transaction_nonce -> Int8,
        transaction_hash -> Text,
        transaction_value -> Int8,
        transaction_block_number -> Int8,
        transaction_block_hash -> Text,
        transaction_block_timestamp -> Timestamptz,
        transaction_block_index -> Int8,
    }
}

diesel::table! {
    wallets (id) {
        id -> Int8,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        chain_id -> Int8,
        wallet_address -> Text,
        block_number -> Int8,
        is_pending -> Bool,
    }
}

diesel::joinable!(groups_permissions -> groups (group_id));
diesel::joinable!(groups_permissions -> permissions (permission_id));
diesel::joinable!(users -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    block_sync,
    groups,
    groups_permissions,
    permissions,
    token_pairs,
    users,
    wallet_token_trades,
    wallet_tokens,
    wallet_transactions,
    wallets,
);
