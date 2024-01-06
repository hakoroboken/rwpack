use rwpack::rwlog_info;
use rwpack::connection::rwudp::{self, UdpData};
use rwpack::timer;
use std::sync::Arc;

use std::{thread, time};


fn hoge(udp_data: &UdpData){
    rwlog_info!("hoge:{}",udp_data.buf[0]);
}

fn hoget(){
    rwlog_info!("timer");
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}

#[tokio::main]
async fn main() {
    rwlog_info!("rwpack start");

    let functions: Arc<[fn(&UdpData); 3]> = Arc::new([hoge,hoge,hoge]);
    let tim_func: Arc<[fn(); 1]> = Arc::new([hoget]);

    let handle1 = rwudp::spawn_rw_udpstream(64201, functions.clone());
    let handle2 = rwudp::spawn_rw_udpstream(64202, functions.clone());
    let handle3 = timer::spawn_rw_timer(1000, tim_func.clone());

    let _ = handle1.await;
    let _ = handle2.await;
    let _ = handle3.await;
}
