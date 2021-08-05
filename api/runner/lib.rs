mod sgx;
mod x86_vm;
mod keystone;

#[cfg(tee_env="sgx")]
use sgx::SgxRunner as Runner;

#[cfg(tee_env="x86_vm")]
use x86_vm::VmRunner as Runner;

#[cfg(tee_env="keystone")]
use keystone::KeystoneRunner as Runner;
