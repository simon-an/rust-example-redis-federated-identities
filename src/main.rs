use log::info;
use redis::Commands;
use std::{env, sync::Arc};

use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredentialBuilder;

async fn connect(sp_object_id: String) -> redis::Connection {
    let creds: Arc<azure_identity::DefaultAzureCredential> = Arc::new(
        DefaultAzureCredentialBuilder::new()
            // .exclude_environment_credential()
            // .exclude_azure_cli_credential()
            // .exclude_managed_identity_credential()
            .build()
            .unwrap(),
    );
    let token = creds
        .get_token(&["acca5fbb-b7e4-4009-81f1-37e38fd66d78/.default"])
        .await // This is the azure resource id for redis
        .unwrap();
    let redis_host_name =
        env::var("REDIS_HOSTNAME").unwrap_or("eon-sm-kvs.redis.cache.windows.net".to_string());
    info!(
        "we have a token and will use it with sp: {} for {redis_host_name}",
        sp_object_id
    );

    let redis_conn_url = format!(
        // "rediss://dc959763-b026-4543-bfd6-e18020e7339f:{}@{}:6380",
        "rediss://{sp_object_id}:{}@{}:6380",
        token.token.secret(),
        redis_host_name
    );
    // println!("redis_conn_url: {}", redis_conn_url);
    redis::Client::open(redis_conn_url)
        .expect("invalid connection URL")
        .get_connection()
        .expect("failed to connect to redis")
}

fn basics(conn: &mut redis::Connection) {
    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(conn)
        .expect("failed to execute SET for 'foo'");

    let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);

    let _: () = conn
        .incr("counter", 2)
        .expect("failed to execute INCR for 'counter'");
    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");
    println!("counter = {}", val);
}

fn list(conn: &mut redis::Connection) {
    let list_name = "items";

    let _: () = redis::cmd("LPUSH")
        .arg(list_name)
        .arg("item-1")
        .query(conn)
        .expect("failed to execute LPUSH for 'items'");

    let item: String = conn
        .lpop(list_name, None)
        .expect("failed to execute LPOP for 'items'");
    println!("first item: {}", item);

    let _: () = conn.rpush(list_name, "item-2").expect("RPUSH failed");
    let _: () = conn.rpush(list_name, "item-3").expect("RPUSH failed");

    let len: isize = conn
        .llen(list_name)
        .expect("failed to execute LLEN for 'items'");
    println!("no. of items in list = {}", len);

    let items: Vec<String> = conn
        .lrange(list_name, 0, len - 1)
        .expect("failed to execute LRANGE for 'items'");

    println!("listing items in list");
    for item in items {
        println!("item: {}", item)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let sp_object_id =
        env::var("AZURE_SP_OBJECT_ID").expect("Missing AZURE_SP_OBJECT_ID environment variable.");
    let mut connection = connect(sp_object_id).await;
    info!("connected to redis");
    info!("basic()");
    basics(&mut connection);
    info!("list()");
    list(&mut connection);
}
