# TUIC-JLS
TUIC protocol based on [JLS](https://github.com/JimmyHuang454/JLS) which enables：
- SNI camouflage
- Anti active detection
- Free of certificate
- Anti hijacking (1RTT only)(TO BE DONE)

# USAGE

## Client
```json5
{
    // Settings for the outbound TUIC proxy
    "relay": {
        // TUIC config
        "server": "example.com:443",
        "uuid": "00000000-0000-0000-0000-000000000000", // Any is OK,TUIC-JLS will skip this
        "password": "PASSWORD",                         // Any is OK
        "ip": "127.0.0.1",
        "udp_relay_mode": "native",
        "congestion_control": "bbr",
        "alpn": ["h3"],
        "zero_rtt_handshake": true,
        "disable_sni": false,
        "timeout": "8s",
        "heartbeat": "3s",
        "disable_native_certs": false,
        "send_window": 16777216,
        "receive_window": 8388608,
        "gc_interval": "3s",
        "gc_lifetime": "15s",

        // JLS password
        "jls_pwd": "123",           // Must be the same as server 
	    "jls_iv":"123",             // Must be the same as server
        // SNI
	    "server_name": "codepen.io" // Must be the same as server jls_upstream
    },

    // Settings for the local inbound socks5 server
    "local": {
        "server": "[::]:1080",
        "username": "USERNAME",
        "password": "PASSWORD",
        "dual_stack": true,
        "max_packet_size": 1500
    },

    "log_level": "warn"
}
```

## Server
```json5
{
    // TUIC config
    "server": "[::]:443",
    "users": {
        "00000000-0000-0000-0000-000000000000": "PASSWORD_0", //Any is ok TUIC-JLS will skip this
    },
    "self_sign": true,  // TUIC-JLS use self-signd certificate
    "congestion_control": "bbr",
    "alpn": ["h3", "spdy/3.1"],
    "udp_relay_ipv6": true,
    "zero_rtt_handshake": false,
    "dual_stack": true,
    "auth_timeout": "3s",
    "task_negotiation_timeout": "3s",
    "max_idle_time": "10s",
    "max_external_packet_size": 1500,
    "send_window": 16777216,
    "receive_window": 8388608,
    "gc_interval": "3s",
    "gc_lifetime": "15s",
    "log_level": "warn",
    
    // JLS password
    "jls_pwd":"123",
    "jls_iv":"123",
    // JLS camouflae server
    "jls_upstream":"codepen.io:443" // port is a must

}
```

# About JLS
[JLS](https://github.com/JimmyHuang454/JLS) is a simple FakeTLS protocol which encodes identity verfication in the Random field of the ClientHello and ServerHello. Its security peformance is the same as tls1.3

# Potential Risk
- see [quinn-jls](https://github.com/spongebob888/quinn-jls)

## See also
- [JLS](https://github.com/JimmyHuang454/JLS) 
- [quinn-jls](https://github.com/spongebob888/quinn-jls)
- [rustls-jls](https://github.com/spongebob888/rustls-jls)
