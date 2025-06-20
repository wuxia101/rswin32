#![cfg(windows)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
use anyhow::{Ok, Result};
use ntapi::ntexapi::{NtQuerySystemInformation, SystemProcessInformation};
use std::ptr;
use winapi::shared::ntdef::{PVOID, ULONG};
use winapi::shared::ntstatus;

use {
    winapi::shared::basetsd::SIZE_T, winapi::shared::ntdef::HANDLE,
    winapi::shared::ntdef::LARGE_INTEGER, winapi::shared::ntdef::ULONGLONG,
    winapi::shared::ntdef::UNICODE_STRING,
};

pub type KPRIORITY = ULONG;
#[repr(C)]
#[derive(Clone)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub NumberOfThreads: ULONG,
    pub WorkingSetPrivateSize: LARGE_INTEGER,
    pub HardFaultCount: ULONG,
    pub NumberOfThreadsHighWatermark: ULONG,
    pub CycleTime: ULONGLONG,
    pub CreateTime: LARGE_INTEGER,
    pub UserTime: LARGE_INTEGER,
    pub KernelTime: LARGE_INTEGER,
    pub ImageName: UNICODE_STRING,
    pub BasePriority: KPRIORITY,
    pub UniqueProcessId: HANDLE,
    pub InheritedFromUniqueProcessId: PVOID,
    pub HandleCount: ULONG,
    pub SessionId: ULONG,
    pub UniqueProcessKey: ULONG,
    pub PeakVirtualSize: SIZE_T,
    pub VirtualSize: SIZE_T,
    pub PageFaultCount: ULONG,
    pub PeakWorkingSetSize: SIZE_T,
    pub WorkingSetSize: SIZE_T,
    pub Reserved5: PVOID,
    pub QuotaPagedPoolUsage: SIZE_T,
    pub Reserved6: PVOID,
    pub QuotaNonPagedPoolUsage: SIZE_T,
    pub PagefileUsage: SIZE_T,
    pub PeakPagefileUsage: SIZE_T,
    pub PrivatePageCount: SIZE_T,
    pub ReadOperationCount: LARGE_INTEGER,
    pub WriteOperationCount: LARGE_INTEGER,
    pub OtherOperationCount: LARGE_INTEGER,
    pub ReadTransferCount: LARGE_INTEGER,
    pub WriteTransferCount: LARGE_INTEGER,
    pub OtherTransferCount: LARGE_INTEGER,
}

pub fn query_system_process_information() -> Result<Vec<SYSTEM_PROCESS_INFORMATION>> {
    // 首先获取需要的内存大小
    let mut return_length: ULONG = 0;
    let _ = unsafe {
        NtQuerySystemInformation(
            SystemProcessInformation,
            ptr::null_mut(),
            0,
            &mut return_length,
        )
    };

    // 分配足够的内存
    let buffer_size = return_length as usize;
    let mut buffer = vec![0u8; buffer_size];

    // 再次调用获取实际数据
    let status = unsafe {
        NtQuerySystemInformation(
            SystemProcessInformation,
            buffer.as_mut_ptr() as PVOID,
            buffer_size as ULONG,
            &mut return_length,
        )
    };
    if status != ntstatus::STATUS_SUCCESS {
        return Err(anyhow::format_err!(
            "Excute query_system_process_information  Error Status {}",
            status
        ));
    }

    // 解析进程信息
    parse_process_information(&buffer)
}

fn parse_process_information(buffer: &[u8]) -> Result<Vec<SYSTEM_PROCESS_INFORMATION>> {
    let mut offset = 0;
    let mut process_count = 0;

    let mut sysprocs = Vec::<SYSTEM_PROCESS_INFORMATION>::with_capacity(1024);

    while offset < buffer.len() {
        let process_info =
            unsafe { &*(buffer.as_ptr().add(offset) as *const SYSTEM_PROCESS_INFORMATION) };

        process_count += 1;

        // 移动到下一个进程信息
        if process_info.NextEntryOffset == 0 {
            break;
        }
        offset += process_info.NextEntryOffset as usize;
        sysprocs.push(process_info.clone());
    }
    Ok(sysprocs)
}
