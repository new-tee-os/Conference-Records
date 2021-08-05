mod keystone;
use keystone::{EnclaveStatus, KeystoneDev};

struct KeystoneRunner{
    memory_manager:EnclaveMemoryManager;
    status:EnclaveStatus;
}

impl KeystoneRunner{
    //KeyStoneDev::open
    //创建页表
    pub fn init(kernel_binary_path: &Path);
    //注册
    pub fn register(edge_call_handler:EdgeCallServer)->bool;
    //finalize+run
    pub fn run(command:&str)->Result<bool,Err>;
    //destroy
    pub fn check_exit()->bool;
}

