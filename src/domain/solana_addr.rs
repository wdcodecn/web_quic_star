use aide::OperationIo;
use anchor_client::anchor_lang::prelude::Pubkey;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(
    OperationIo, Default, Debug, Clone, AsExpression, FromSqlRow, Copy, Hash, Eq, PartialEq,
)]
#[diesel(sql_type = Text)]
pub struct SolAddr(pub Pubkey);

impl From<Pubkey> for SolAddr {
    fn from(value: Pubkey) -> Self {
        SolAddr(value)
    }
}
impl Serialize for SolAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SolAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use std::str::FromStr;
        String::deserialize(deserializer)
            .and_then(|string| {
                Pubkey::from_str(&string).map_err(|err| {
                    Error::custom(format!("deserialize value:`{string}` failed,err:{err}"))
                })
            })
            .map(SolAddr)
    }
}

impl JsonSchema for SolAddr {
    fn schema_name() -> String {
        "PubKey".to_owned()
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

impl ToSql<Text, Pg> for SolAddr {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <String as ToSql<VarChar, Pg>>::to_sql(&self.0.to_string(), &mut out.reborrow())
    }
}

impl FromSql<Text, Pg> for SolAddr {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let string = <String as FromSql<VarChar, Pg>>::from_sql(bytes)?;
        let pubkey = Pubkey::from_str(&string).map_err(Box::new)?;

        Ok(SolAddr(pubkey))
    }
}

impl AsRef<Pubkey> for SolAddr {
    fn as_ref(&self) -> &Pubkey {
        &self.0
    }
}

impl Deref for SolAddr {
    type Target = Pubkey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SolAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
