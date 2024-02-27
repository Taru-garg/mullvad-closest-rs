use clap::{self, Parser};
use futures::future::join_all;
use mullvad_closest_rs::structs::{Args, Server};
use tabled::{
    settings::{Alignment, Style},
    Table,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut servers: Vec<Server> = Server::get_relevant_servers_from_config(args).await;
    let mut tasks = vec![];
    for server in &mut servers {
        tasks.push(server.ping());
    }

    join_all(tasks).await;

    servers.sort_unstable_by(|lhs, rhs| lhs.latency.partial_cmp(&rhs.latency).unwrap());
    servers = servers
        .into_iter()
        .filter(|x| x.latency != f64::MAX) // Don't display the Server which failed to send a resp back
        .collect();

    println!(
        "{}",
        Table::new(servers)
            .with(Style::psql())
            .with(Alignment::left())
    );

    Ok(())
}
