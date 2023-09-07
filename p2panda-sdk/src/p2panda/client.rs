use std::any::type_name;

use anyhow::Result;
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use p2panda_rs::{
    document::DocumentViewId,
    entry::{encode::sign_and_encode_entry, traits::AsEncodedEntry},
    identity::KeyPair,
    operation::{encode::encode_plain_operation, plain::PlainOperation, traits::Actionable},
};

use crate::p2panda::graphql::{
    NextArgsQuery, NextArgsVaribles, NextArguments, PublishMutation, PublishVariables,
};

pub struct Client {
    client: reqwest::Client,
    key_pair: KeyPair,
    endpoint: &'static str,
}

impl Client {
    pub fn new(key_pair: KeyPair, endpoint: &'static str) -> Self {
        Self {
            client: reqwest::Client::new(),
            key_pair,
            endpoint,
        }
    }
    async fn query_next_args(
        &self,
        view_id: Option<DocumentViewId>,
    ) -> Result<Option<NextArguments>> {
        let query = NextArgsQuery::build(NextArgsVaribles {
            public_key: self.key_pair.public_key(),
            view_id,
        });

        let resp = self.handle_graphql(query).await?;

        Ok(resp.next_args)
    }

    async fn handle_graphql<Vars, Resp>(
        &self,
        operation: cynic::Operation<Resp, Vars>,
    ) -> Result<Resp>
    where
        Vars: serde::Serialize,
        Resp: serde::de::DeserializeOwned + 'static,
    {
        let res = self
            .client
            .post(self.endpoint)
            .run_graphql(operation)
            .await?;
        if let Some(errors) = res.errors {
            eprintln!("{errors:?}");
        }
        if let Some(res) = res.data {
            return Ok(res);
        }
        Err(anyhow::anyhow!("Query {:?} failed.", type_name::<Resp>()))
    }

    pub async fn publish(&self, operation: PlainOperation) -> Result<NextArguments> {
        let Some(next_args) = self.query_next_args(operation.previous().cloned()).await? else {
            return Err(anyhow::anyhow!("GraphQL query to fetch `nextArgs` failed"));
        };

        let encoded_operation = encode_plain_operation(&operation)?;
        let encoded_entry = sign_and_encode_entry(
            &next_args.log_id,
            &next_args.seq_num,
            next_args.skiplink.as_ref(),
            next_args.backlink.as_ref(),
            &encoded_operation,
            &self.key_pair,
        )?;

        println!("▶ Operation Id: \"{}\"", encoded_entry.hash());

        let mutation = PublishMutation::build(PublishVariables {
            entry: encoded_entry,
            operation: encoded_operation,
        });

        let resp = self.handle_graphql(mutation).await?;

        println!("\nWoho! ヽ(￣(ｴ)￣)ﾉ");

        Ok(resp.publish)
    }

    // TODO: 等待新版本blob相关的内容发布
    // pub async fn publish_blob(&self,file_path: &Path) -> Result<()> {
    //     let file = File::open(file_path).await?;
    //     let metadata = file.metadata().await?;
    //     let file_size = metadata.len();
    //     let mime_type = match mime_guess::from_path(file_path).first() {
    //         Some(guessed_type) => guessed_type.to_string(),
    //         None => "application/octet-stream".into(),
    //     };
    //     let expected_blob_pieces = file_size / MAX_BLOB_PIECE_LENGTH as u64;
    //     Ok(())
    // }
}
