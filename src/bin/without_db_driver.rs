fn main() {
    // fn create_tx_with_address_table_lookup(
    //     client: &RpcClient,
    //     instructions: &[Instruction],
    //     address_lookup_table_key: Pubkey,
    //     payer: &Keypair,
    // ) -> Result<VersionedTransaction> {
    //     let raw_account = client.get_account(&address_lookup_table_key)?;
    //     let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    //     let address_lookup_table_account = AddressLookupTableAccount {
    //         key: address_lookup_table_key,
    //         addresses: address_lookup_table.addresses.to_vec(),
    //     };
    //
    //     let blockhash = client.get_latest_blockhash()?;
    //     let tx = VersionedTransaction::try_new(
    //         VersionedMessage::V0(v0::Message::try_compile(
    //             &payer.pubkey(),
    //             instructions,
    //             &[address_lookup_table_account],
    //             blockhash,
    //         )?),
    //         &[payer],
    //     )?;
    //
    //     assert!(tx.message.address_table_lookups().unwrap().len() > 0);
    //     Ok(tx)
    // }
    // let manager = PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);
    // let pool = r2d2::Pool::new(manager).unwrap();
    // let new = GroupsPermission { group_id: 0, permission_id: 0 };
    // let mut client = pool.get().unwrap();

    // diesel::insert_into(groups_permissions).values(new).execute(&mut client);
    // for i in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    // client
    //     .execute("INSERT INTO groups_permissions  VALUES (9,9)", &[])
    //     .unwrap();
    //     });
    // }
}
