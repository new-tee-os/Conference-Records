## TEEOS Runner Crate API 设计 ##

- SGX模块:
整个内核通过Makefile编译成一个enclave的动态链接库，用xml配置文件进行配置
必要的build.rs
1. 分配共享内存
2. 把用户elf加载到共享内存中
3. 初始化 sgx enclave 

4. 运行，调用ecall进行（用户自定义）

5. *其中exception在内部处理

6. 检查enclave退出情况

- vm模块
内核编译成一个bin文件，kernel定义entry_point
1. 初始化 bootloader包创建img；创建qemu运行的disk空间

2. 运行 qemu命令行

3. 外部监听edgecall（通过edge.sock这一socket异步通信）

4. 检查退出状态

- keystone
内核编译成一个bin文件，手动配置内存参数
1. 初始化 open enclave；把bin写入enclave
2. 创建页表

3. 运行 finalize+run

4. 外部监听edgecall

5. 检查退出状态



