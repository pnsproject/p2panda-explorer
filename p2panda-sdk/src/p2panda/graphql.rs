use p2panda_rs::{
    document::DocumentViewId,
    entry::{EncodedEntry, LogId, SeqNum},
    hash::Hash,
    identity::PublicKey,
    operation::EncodedOperation,
};

#[derive(cynic::QueryVariables, Debug)]
pub struct NextArgsVaribles {
    pub public_key: PublicKey,
    pub view_id: Option<DocumentViewId>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "NextArgsVaribles")]
pub struct NextArgsQuery {
    #[arguments(publicKey: $public_key, viewId: $view_id)]
    pub next_args: Option<NextArguments>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct NextArguments {
    pub log_id: LogId,
    pub seq_num: SeqNum,
    pub skiplink: Option<Hash>,
    pub backlink: Option<Hash>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PublishVariables {
    pub entry: EncodedEntry,
    pub operation: EncodedOperation,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "PublishVariables")]
pub struct PublishMutation {
    #[arguments(entry: $entry, operation: $operation)]
    pub publish: NextArguments,
}

#[cynic::schema("p2panda")]
mod schema {}

cynic::impl_scalar!(DocumentViewId, schema::DocumentViewId);

cynic::impl_scalar!(Hash, schema::EntryHash);

cynic::impl_scalar!(LogId, schema::LogId);

cynic::impl_scalar!(PublicKey, schema::PublicKey);

cynic::impl_scalar!(SeqNum, schema::SeqNum);

cynic::impl_scalar!(EncodedEntry, schema::EncodedEntry);

cynic::impl_scalar!(EncodedOperation, schema::EncodedOperation);