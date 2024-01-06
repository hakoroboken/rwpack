use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::sync::Arc;

use crate::{rwlog_info, rwlog_error};

pub struct UdpData {
    pub len: usize,
    pub addr: SocketAddr,
    pub buf: [u8; 1024],
}

pub fn spawn_rw_udpstream(port: u16, functions: Arc<[fn(&UdpData)]>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        rw_udpstream(port, functions).await;
    })
}


async fn rw_udpstream(port: u16, functions: Arc<[fn(&UdpData)]>) {
    rwlog_info!("rw_udpstream start. port:{}",port);
    let sock = UdpSocket::bind("0.0.0.0:".to_string() + &port.to_string()).await.unwrap();
    let mut udp_data = UdpData {
        len: 0,
        addr: "0.0.0.0:0".parse().unwrap(),
        buf: [0; 1024],
    };
    
    loop {
        match sock.recv_from(&mut udp_data.buf).await {
            Ok((len, addr)) => {
                udp_data.len = len;
                udp_data.addr = addr;
                rwlog_info!("{}バイトが{}から受信されました。", udp_data.len, udp_data.addr);

                for function in functions.as_ref() {
                    function(&udp_data);
                }
            },
            Err(e) => {
                rwlog_error!("UDP受信中にエラーが発生しました: {}", e);
                panic!("UDP Error");
            }
        }
    }
}