use alloy_sol_types::sol;
use futures::Future;
use reth_exex::ExExContext;
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_primitives::{address, Address};
use reth_tracing::tracing::info;
use rusqlite::Connection;

sol! {
    interface ScrollChain {
        event CommitBatch(uint256 indexed batchIndex, bytes32 indexed batchHash);
        event RevertBatch(uint256 indexed batchIndex, bytes32 indexed batchHash);
    }
}

#[allow(dead_code)]
const SCROLL_CHAIN_ADDRESS: Address = address!("a13BAF47339d63B743e7Da8741db5456DAc1E556");

fn create_tables(connection: &mut Connection) -> rusqlite::Result<()> {
    connection.execute(
        r#"
            CREATE TABLE IF NOT EXISTS batches (
                index  INTEGER PRIMARY KEY,
                hash   TEXT NOT NULL
            );
            "#,
        (),
    )?;

    info!("Initialized database tables");
    Ok(())
}

async fn scroll_indexer_exex<Node: FullNodeComponents>(
    mut _ctx: ExExContext<Node>,
    _connection: Connection,
) -> eyre::Result<()> {
    unimplemented!()
}

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
    mut connection: Connection,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    create_tables(&mut connection)?;
    Ok(scroll_indexer_exex(ctx, connection))
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("ScrollIndexer", |ctx| async move {
                let connection = Connection::open("scroll_indexer.db")?;
                exex_init(ctx, connection).await
            })
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}
