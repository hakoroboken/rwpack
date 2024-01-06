use rwpack::rwlog_info;
use rwpack::connection::rwudp::{self, UdpData};
use std::sync::Arc;

fn hoge(udp_data: &UdpData){
    rwlog_info!("hoge:{}",udp_data.buf[0]);
}

#[tokio::main]
async fn main() {
    rwlog_info!("rwpack start");

    let functions: Arc<[fn(&UdpData)]> = Arc::new([hoge,hoge,hoge]);


    let handle1 = rwudp::spawn_rw_udpstream(64201, functions.clone());
    let handle2 = rwudp::spawn_rw_udpstream(64202, functions.clone());

    let _ = handle1.await;
    let _ = handle2.await;
}
