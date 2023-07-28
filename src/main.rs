use log::info;
use redis::Commands;
use std::{env, sync::Arc};

use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredentialBuilder;

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
