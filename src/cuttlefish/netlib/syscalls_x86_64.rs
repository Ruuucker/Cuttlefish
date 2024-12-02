#![allow(non_snake_case)]
use core::arch::asm;
use crate::netlib::*;

#[cfg(not(target_os = "linux"))]
compile_error!("Only works on Linux");

/*
Source
https://livegrep.com/
https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/socket.h#L183
https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/net.h#L49
https://github.com/torvalds/linux/blob/3d7cb6b0/include/uapi/linux/in.h#L31

Structs and consts
*/


// Syscalls
unsafe fn sys_setsockopt(num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: *const [u32;1], arg5: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        lateout("rax") res,
    );
    res
}

pub fn setsockopt(fd: u64, level: u64, option_name: u64, option_value: *const [u32;1], option_len: u64) -> i64 {
    unsafe { sys_setsockopt(54 as u64, fd, level, option_name, option_value, option_len) }
}


// asmlinkage long sys_recvmsg(int fd, struct user_msghdr __user *msg, unsigned flags);
unsafe fn sys_recvmsg(num: u64, arg1: u64, arg2: *const msghdr, arg3: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res,
    );
    res
}


pub fn recvmsg(fd: u64, msghdr: &msghdr, flags: u64) -> i64 {
    unsafe { sys_recvmsg(47 as u64, fd, msghdr, flags) }
}

unsafe fn sys_socket(num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res,
    );
    res
}

unsafe fn sys_connect(num: u64, arg1: u64, arg2: *const sockaddr, arg3: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res,
    );
    res
}

pub fn sendto(fd: u64, data: *const u8, len: u64, flags: u64, sockaddr: &sockaddr, addrlen: u64) -> i64 {
    unsafe { sys_sendto(44 as u64, fd, data, len, flags, sockaddr, addrlen) }
}

unsafe fn sys_sendto(num: u64, arg1: u64, arg2: *const u8, arg3: u64, arg4: u64, arg5: *const sockaddr, arg6: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        lateout("rax") res,
    );
    res
}

pub fn socket(fd: u64, data: u64, len: u64) -> i64 {
    unsafe { sys_socket(41 as u64, fd, data as u64, len) }
}

pub fn connect(fd: u64, data: &sockaddr, len: u64) -> i64 {
    unsafe { sys_connect(42 as u64, fd, data, len) }
}

unsafe fn sys_fcntl(num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res,
    );
    res
}

pub fn fcntl(fd: u64, cmd: u64, flags: u64) -> i64 {
    unsafe { sys_fcntl(72 as u64, fd, cmd, flags) }
}

unsafe fn sys_close(num: u64, arg1: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        lateout("rax") res,
    );
    res
}

pub fn close(fd: u64) -> i64 {
    unsafe { sys_close(3 as u64, fd) }
}

/*
unsafe fn sys_getrlimit(num: u64, arg1: u64, arg2: *const rlimit) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rax") res,
    );
    res
}
*/

unsafe fn sys_rlimit(num: u64, arg1: u64, arg2: *const rlimit) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rax") res,
    );
    res
}

//pub fn getrlimit(resource: u64, rlim: &rlimit) -> i64 {
//    unsafe { sys_rlimit(97 as u64, resource, rlim) }
//}

pub fn setrlimit(resource: u64, rlim: &rlimit) -> i64 {
    unsafe { sys_rlimit(160 as u64, resource, rlim) }
}


pub fn epoll_create1(flags: u64) -> i64 {
    unsafe { sys_epoll_create1(291 as u64, flags) }
}

unsafe fn sys_epoll_create1(num: u64, arg1: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        lateout("rax") res,
    );
    res
}

pub fn epoll_ctl(epfd: u64, op: u64, fd: u64, epoll_event: &epoll_event) -> i64 {
    unsafe { sys_epoll_ctl(233 as u64, epfd, op, fd, epoll_event) }
}

unsafe fn sys_epoll_ctl(num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: *const epoll_event) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        lateout("rax") res,
    );
    res
}

pub fn epoll_pwait(epfd: u64, epoll_event: *mut epoll_event, maxevents: u64, timeout: u64) -> i64 {
    unsafe { sys_epoll_pwait(232 as u64, epfd, epoll_event, maxevents, timeout, 0) }
}

unsafe fn sys_epoll_pwait(num: u64, arg1: u64, arg2: *const epoll_event, arg3: u64, arg4: u64, arg5: u64) -> i64 {
    let res;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        lateout("rax") res,
    );
    res
}
