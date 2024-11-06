use aide::OperationIo;
use alloy::primitives::Address;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

#[derive(OperationIo, Default, Debug, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct EthAddr(pub Address);

impl From<Address> for EthAddr {
    fn from(value: Address) -> Self {
        EthAddr(value)
    }
}

impl Serialize for EthAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for EthAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use std::str::FromStr;
        String::deserialize(deserializer)
            .and_then(|string| {
                Address::from_str(&string).map_err(|err| {
                    Error::custom(format!("serialize value:`{string}` failed,err:{err}"))
                })
            })
            .map(|x| EthAddr(x))
    }
}

impl AsRef<Address> for EthAddr {
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl Deref for EthAddr {
    type Target = Address;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EthAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JsonSchema for EthAddr {
    fn schema_name() -> String {
        "SolanaAddr".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;

impl ToSql<Text, Pg> for EthAddr {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <String as ToSql<VarChar, Pg>>::to_sql(&self.0.to_string(), &mut out.reborrow())
    }
}

impl FromSql<Text, Pg> for EthAddr {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        <String as FromSql<VarChar, Pg>>::from_sql(bytes).map(|s| {
            EthAddr(
                s.parse()
                    .expect("Failed to parsing raw_sql_type to Ethereum address"),
            )
        })
    }
}