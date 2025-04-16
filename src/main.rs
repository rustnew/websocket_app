use warp::Filter;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
mod func;

#[tokio::main]
async fn main() {
    // Initialisation du canal de diffusion pour les messages
    let (tx, _rx) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    // Configuration de la route WebSocket
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_tx(tx.clone()))
        .map(|ws: warp::ws::Ws, tx| {
            ws.on_upgrade(move |socket| func::handle_connection(socket, tx))
        });

    // Lancement du serveur sur le port 8080
    println!("🚀 Serveur en cours d'exécution sur ws://127.0.0.1:8080/ws");
    warp::serve(ws_route).run(([127, 0, 0, 1], 8082)).await;
}

// Fonction utilitaire pour passer le canal de diffusion à la route
fn with_tx(
    tx: Arc<Mutex<broadcast::Sender<String>>>,
) -> impl Filter<Extract = (Arc<Mutex<broadcast::Sender<String>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}
