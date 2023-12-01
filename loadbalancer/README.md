### Load Balancer nginx - docker-compose.yml


---

- nginx.conf 
```sh

events{
}
http {
  upstream myproject {
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
    server 127.0.0.1:8083;
  }
  server {
    listen 8080;
    listen [::]:8080;    
    server_name  _;
    location / {
      proxy_pass http://myproject;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;      
    }
  }
  server {
    listen 8081;
    location / {
      return 200 "<h1>Server 1 - AppServer 1</h1>\n";
    }
  }
  server {
    listen 8082;
    location / {
      return 200 "<h1>Server 1 - AppServer 2</h1>\n";
    }
  }
  server {
    listen 8083;
    location / {
      return 200 "<h1>Server 1 - AppServer 3</h1>\n";
    }
  }
###### Second Server
  upstream myproject2 {
    server 127.0.0.1:8091;
    server 127.0.0.1:8092;
    server 127.0.0.1:8093;
  }
  server {
    listen 8080;
    listen [::]:8080;    
    server_name  app2.srv2;
    location / {
      proxy_pass http://myproject2;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;      
    }
  }
  server {
    listen 8091;
    location / {
      return 200 "<h1>Server 2 - AppServer 1</h1>\n";
    }
  }
  server {
    listen 8092;
    location / {
      return 200 "<h1>Server 2 - AppServer 2</h1>\n";
    }
  }
  server {
    listen 8093;
    location / {
      return 200 "<h1>Server 2 - AppServer 3</h1>\n";
    }
  }
}

```
-  docker-compose.yml
```yml
version : “3”
services :
  nginx:
    image: nginx:latest
    container_name: nginx_container_loadbalancer
    ports:
      - 80:80
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    network_mode: host

```
-  start loadbalancer server nginx over docker-compose
```sh
┌──(kali㉿kali)-[~/projects/weekly66/loadbalancer]
└─$ docker compose up
[+] Running 1/0
 ✔ Container nginx_container_loadbalancer  Created                                                                                             0.0s 
Attaching to nginx_container_loadbalancer
nginx_container_loadbalancer  | /docker-entrypoint.sh: /docker-entrypoint.d/ is not empty, will attempt to perform configuration
nginx_container_loadbalancer  | /docker-entrypoint.sh: Looking for shell scripts in /docker-entrypoint.d/
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/10-listen-on-ipv6-by-default.sh
nginx_container_loadbalancer  | 10-listen-on-ipv6-by-default.sh: info: IPv6 listen already enabled
nginx_container_loadbalancer  | /docker-entrypoint.sh: Sourcing /docker-entrypoint.d/15-local-resolvers.envsh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/20-envsubst-on-templates.sh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/30-tune-worker-processes.sh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Configuration complete; ready for start up
```

- verify access servers
```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/
<h1>Server 1 - AppServer 3</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://app1.srv1:8080/
<h1>Server 1 - AppServer 1</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://app1.srv1:8080/
<h1>Server 1 - AppServer 2</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://app2.srv2:8080/
<h1>Server 2 - AppServer 2</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://app2.srv2:8080/
<h1>Server 2 - AppServer 3</h1>
```


---

```sh

events{
}
http {

  upstream myproject {
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
    server 127.0.0.1:8083;
  }

  server {
    listen 8080;
    listen [::]:8080;    
    location / {
      proxy_pass http://myproject;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;      
    }
  }

  server {
    listen 8081;
    location / {
      return 200 "<h1>AppServer 1</h1>\n";
    }
  }

  server {
    listen 8082;
    location / {
      return 200 "<h1>AppServer 2</h1>\n";
    }
  }

  server {
    listen 8083;
    location / {
      return 200 "<h1>AppServer 3</h1>\n";
    }
  }
}


```

```yml
version : “3”
services :
  nginx:
    image: nginx:latest
    container_name: nginx_container_loadbalancer
    ports:
      - 80:80
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    network_mode: host

```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/loadbalancer]
└─$ docker compose up              
[+] Running 1/0
 ✔ Container nginx_container_loadbalancer  Created                                                                                             0.0s 
Attaching to nginx_container_loadbalancer
nginx_container_loadbalancer  | /docker-entrypoint.sh: /docker-entrypoint.d/ is not empty, will attempt to perform configuration
nginx_container_loadbalancer  | /docker-entrypoint.sh: Looking for shell scripts in /docker-entrypoint.d/
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/10-listen-on-ipv6-by-default.sh
nginx_container_loadbalancer  | 10-listen-on-ipv6-by-default.sh: info: IPv6 listen already enabled
nginx_container_loadbalancer  | /docker-entrypoint.sh: Sourcing /docker-entrypoint.d/15-local-resolvers.envsh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/20-envsubst-on-templates.sh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Launching /docker-entrypoint.d/30-tune-worker-processes.sh
nginx_container_loadbalancer  | /docker-entrypoint.sh: Configuration complete; ready for start up
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/ 
<h1>AppServer 1</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/
<h1>AppServer 2</h1>
                                                                                                                                                    
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/
<h1>AppServer 3</h1>


┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/ && curl http://localhost:8080/ && curl http://localhost:8080/ && curl http://localhost:8080/
<h1>AppServer 2</h1>
<h1>AppServer 3</h1>
<h1>AppServer 1</h1>
<h1>AppServer 2</h1>

```

---


```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ sudo apt -y install nginx


┌──(kali㉿kali)-[~/projects/weekly66]
└─$ cat /etc/nginx/nginx.conf 
user www-data;
worker_processes auto;
pid /run/nginx.pid;
error_log /var/log/nginx/error.log;
include /etc/nginx/modules-enabled/*.conf;

events {
        worker_connections 768;
        # multi_accept on;
}

http {

  server {
    listen 80;
    listen [::]:80;
    location / {
        proxy_pass http://127.0.0.1:3030;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
  }
}

```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ sudo systemctl start nginx

┌──(kali㉿kali)-[~/projects/weekly66]
└─$ systemctl status nginx         
● nginx.service - A high performance web server and a reverse proxy server
     Loaded: loaded (/lib/systemd/system/nginx.service; disabled; preset: disabled)
     Active: active (running) since Wed 2023-11-29 15:47:05 CET; 22min ago
     ...
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost/hello
<h1>Working Rust! Default!</h1>   
```




