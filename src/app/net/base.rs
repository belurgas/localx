// Скопировать в будущем код из PopkaNet/pips

use pnet::datalink::{interfaces, NetworkInterface};

pub fn get_interface() -> NetworkInterface {
    let interfaces = interfaces()[0].clone();
    return interfaces;

}
