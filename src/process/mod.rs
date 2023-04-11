//! Custom Process API

const QEMU_PORT: u16 = 0xf4 as u16;

#[unstable(feature = "std3_process", issue = "none")]
/// Exits Qemu Execution, requires `qemu` feature
pub unsafe fn exit_qemu(code: u8){
    match crate::io::write_to_port(QEMU_PORT, code){
        Ok(_) => (),
        Err(e) => {panic!("{}",e)}
    }
}