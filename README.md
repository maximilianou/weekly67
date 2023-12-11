# weekly67


- Connect to our server 
```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ ssh dev01@srv02.simpledoers.com
```
- Check the kubeconfig file on our server
```yml
root@srv2:~# cat /var/lib/k0s/pki/admin.conf 
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: ..
    server: https://localhost:6443
  name: local
contexts:
- context:
    cluster: local
    user: user
  name: Default
current-context: Default
kind: Config
preferences: {}
users:
- name: user
..
```

- Copy the kubeconfig to our local computer
```yml
┌──(kali㉿kali)-[~/projects]
└─$ cat ~/.kube/config2
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: ..
    server: https://localhost:16443
  name: local
contexts:
- context:
    cluster: local
    user: user
  name: Default
current-context: Default
kind: Config
preferences: {}
users:
- name: user
  user:
  ..
```

- SSH Tunneling - create a tunnel in localport 16443 to the server srv02.simpledoers.com to access internaly to localhost:6443
```sh
┌──(kali㉿kali)-[~]
└─$ ssh -NL 16443:localhost:6443 dev01@srv02.simpledoers.com
dev01@srv02.simpledoers.com's password: 

```

- Accesso to kubectl over the kubeconfig to the remote server
```sh
┌──(kali㉿kali)-[~/projects]
└─$ KUBECONFIG=/home/kali/.kube/config2 ; kubectl get all -A
```

```sh
┌──(kali㉿kali)-[~/projects]
└─$ KUBECONFIG=/home/kali/.kube/config2 ; kubectl get all   
NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   4d5h
```

