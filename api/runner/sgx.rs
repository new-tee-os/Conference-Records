use sgx_types::*;
use sgx_urts::SgxEnclave;

struct SgxRunner{
    shared_mem_size:u8;
    shared_mem:*mut u8;
    result:sgx_status_t;
    enclave:SgxEnclave;
    pub start_point:fn(*mut u8, usize) -> sgx_status_t;
}
impl SgxRunner{
    pub fn init(shared_mem_size:u8,elf_path:&str) -> SgxResult<SgxEnclave>;s
    
    pub fn run()-> sgx_status_t;
    
    pub fn check_exit()->bool{
        match result {
            sgx_status_t::SGX_SUCCESS => {},
            _ => {
                println!("[-] ECALL Enclave Failed {}!", result.as_str());
                return;
            }
        }
        println!("[+] Exit success...");
        enclave.destroy();
    }
}
