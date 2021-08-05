use std::path::{Path, PathBuf};
use EdgeCallServer;

struct VmRunner{
    disk_img:&str;
    //使用EdgeCallServer，可以考虑做成trait
    result:EdgeCallServer;
}

impl VmRunner{
    pub async fn init(kernel_binary_path: &Path) -> PathBuf;
    pub fn register(edge_call_handler:EdgeCallServer)->bool;

    //在run中添加检查回调的代码
    pub fn run(command:&str)->Result<bool,Err>;
    pub fn check_exit()->bool;
}