# 安装证书

https://github.com/FiloSottile/mkcert
https://github.com/FiloSottile/mkcert/releases

```
$ mkcert -install
Created a new local CA 💥
The local CA is now installed in the system trust store! ⚡️
The local CA is now installed in the Firefox trust store (requires browser restart)! 🦊

$ mkcert localhost localhost 127.0.0.1 ::1 172.65.11.22

Created a new certificate valid for the following names 📜
 - "localhost"
 - "127.0.0.1"
 - "::1"

The certificate is at "./localhost+3.pem" and the key at "./localhost+3-key.pem" 
```