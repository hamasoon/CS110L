# Error

## Error Handling

#### in C
- Check error by using integer return value(0, -1...)
- Or using NULL, nullptr for pointer handling function
```c
ssize_t siz = msgrcv(msqid, msgp, msgsz, msgtyp, msgflg);
if (siz<0) { // msgrcv failed and has set errno
  if (errno == ENOMSG) 
    dosomething();
  else if (errno == EAGAIN) 
    dosomethingelse();
  /// etc
  else {  
     syslog(LOG_DAEMON|LOG_ERR, "msgrcv failure with %s\n",
            strerror(errno)); 
     exit(EXIT_FAILURE); 
  };
};
```

#### Exception(JAVA, Python)
- Failure modes are hard to spot: any function can throw any exception at any time
- in Rust
  - `panic!` macro for unrecoverable error
  - `Result<T, E>`: `Ok(T)` or `Err(E)` for recoverable error
    - handle it by using unwrap(), expect(), match(highly recommended)
  - Sometime, `Option<T>` is used for recoverable error(for represent NULL value)