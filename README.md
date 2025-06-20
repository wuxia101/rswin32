# rswin32

Windows utility functions implemented by Rust.

```
cargo run --bin query_ps_info
```

```
正在查询系统进程信息...
找到 361 个进程:
PID        进程名                  线程数             内存使用
----------------------------------------------------------------------
0          System Idle Process  4               8 KB
4          System               238             6.4 MB
56         Secure System        0               38.1 MB
116        Registry             4               63.4 MB
444        smss.exe             2               232 KB
616        csrss.exe            11              2.0 MB
696        wininit.exe          1               640 KB
704        csrss.exe            16              4.0 MB
768        winlogon.exe         7               4.0 MB
840        services.exe         9               7.1 MB
860        LsaIso.exe           1               948 KB
868        lsass.exe            9               14.7 MB
996        svchost.exe          16              23.5 MB
80         fontdrvhost.exe      5               488 KB
364        fontdrvhost.exe      5               6.8 MB
948        svchost.exe          12              16.9 MB
1036       svchost.exe          7               4.6 MB
1120       svchost.exe          43              5.2 MB
1164       svchost.exe          2               1.1 MB
1220       svchost.exe          2               1.8 MB
... 还有 341 个进程未显示
```

参考
- https://github.com/heim-rs/heim
- https://github.com/likeanocean/ari
- https://github.com/hinaria/ntdll