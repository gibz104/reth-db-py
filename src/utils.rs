use reth_db::open_db_read_only;
use reth_primitives::ChainSpecBuilder;
use reth_provider::{BlockReader, HeaderProvider, ProviderFactory, ReceiptProvider, TransactionsProvider};

use std::path::Path;
use std::sync::Arc;
use reth_db::DatabaseEnvRO;

pub struct DatabaseHandler {
    factory: ProviderFactory<DatabaseEnvRO>,
}

impl DatabaseHandler {
    pub fn new(db_path: String) -> eyre::Result<Self> {
        let db = open_db_read_only(&Path::new(&db_path), None)?;
        let spec = Arc::new(ChainSpecBuilder::mainnet().build());
        let factory = ProviderFactory::new(db, spec.clone());
        Ok(Self { factory })
    }

    pub(crate) fn get_header_by_block_number(&self, number: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let header = provider.header_by_number(number)?;
        let json = serde_json::to_string(&header)?;
        Ok(json)
    }

    pub(crate) fn get_headers_by_block_number_range(&self, start: u64, end: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let headers = provider.sealed_headers_range(&start..&end)?;
        let json = serde_json::to_string(&headers)?;
        Ok(json)
    }

    pub(crate) fn get_transaction_by_id(&self, id: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let transaction = provider.transaction_by_id(id)?;
        let json = serde_json::to_string(&transaction)?;
        Ok(json)
    }

    pub(crate) fn get_transactions_by_id_range(&self, start: u64, end: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let transactions = provider.transactions_by_tx_range(&start..&end)?;
        let json = serde_json::to_string(&transactions)?;
        Ok(json)
    }

    pub(crate) fn get_transactions_by_block_number_range(&self, start: u64, end: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let transactions = provider.transactions_by_block_range(&start..&end)?;
        let json = serde_json::to_string(&transactions)?;
        Ok(json)
    }

    pub(crate) fn get_block_by_number(&self, number: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let block = provider.block(number.into())?;
        let json = serde_json::to_string(&block)?;
        Ok(json)
    }

    pub(crate) fn get_uncles_by_block_number(&self, number: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let uncles = provider.ommers(number.into())?;
        let json = serde_json::to_string(&uncles)?;
        Ok(json)
    }

    pub(crate) fn get_receipts_by_transaction_id(&self, id: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let receipts = provider.receipt(id)?;
        let json = serde_json::to_string(&receipts)?;
        Ok(json)
    }

    pub(crate) fn get_receipts_by_block_number(&self, number: u64) -> eyre::Result<String> {
        let provider = self.factory.provider()?;
        let receipts = provider.receipts_by_block(number.into())?;
        let json = serde_json::to_string(&receipts)?;
        Ok(json)
    }
}
