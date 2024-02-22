
### Function calling conventions in C:

Calling conventions specify the details of a function call. For example, they specify where function parameters are placed (e.g. in registers or on the stack) and how results are returned. On x86_64 Linux, the following rules apply for C functions (specified in the System V ABI):

The first six integer arguments are passed in registers rdi, rsi, rdx, rcx, r8, r9
additional arguments are passed on the stack
results are returned in rax and rdx


### Preserved and Scratch Registers

The calling convention divides the registers into two parts: preserved and scratch registers.

The values of preserved registers must remain unchanged across function calls. So a called function (the “callee”) is only allowed to overwrite these registers if it restores their original values before returning. Therefore, these registers are called “callee-saved”. A common pattern is to save these registers to the stack at the function’s beginning and restore them just before returning.

In contrast, a called function is allowed to overwrite scratch registers without restrictions. If the caller wants to preserve the value of a scratch register across a function call, it needs to backup and restore it before the function call (e.g., by pushing it to the stack). So the scratch registers are caller-saved.