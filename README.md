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



------------
------------

### fluxcd

https://fluxcd.io/flux/get-started/

```sh
curl -s https://fluxcd.io/install.sh | bash
```

```sh
kubectl apply -f https://github.com/fluxcd/flux2/releases/latest/download/install.yaml
```

```sh
export GITHUB_TOKEN=<gh-token>
```

```sh
dev01@srv11:~$ flux bootstrap github --token-auth --owner=maximilianou --repository=https://github.com/maximilianou/weekly67/ --branch=main --path=clusters/my-cluster --personal
```

```sh
dev01@srv11:~$ flux bootstrap github --token-auth --owner=maximilianou --repository=https://github.com/maximilianou/weekly67/ --branch=main --path=clusters/my-cluster --personal
✗ cluster info unavailable: failed to get API group resources: unable to retrieve the complete list of server APIs: apiextensions.k8s.io/v1: Get "http://localhost:8080/apis/apiextensions.k8s.io/v1": dial tcp [::1]:8080: connect: connection refused
dev01@srv11:~$ kubectl get all
WARN[0000] Unable to read /etc/rancher/k3s/k3s.yaml, please start server with --write-kubeconfig-mode to modify kube config permissions 
error: error loading config file "/etc/rancher/k3s/k3s.yaml": open /etc/rancher/k3s/k3s.yaml: permission denied
dev01@srv11:~$ ls /tmp
systemd-private-a56373d70a9e4626aea13fe7329f73ee-systemd-logind.service-BvrXaz
systemd-private-a56373d70a9e4626aea13fe7329f73ee-systemd-timesyncd.service-dFLwOn
dev01@srv11:~$ su -
Password: 
su: Authentication failure
dev01@srv11:~$ su -
Password: 
root@srv11:~# cat /etc/rancher/k3s/k3s.yaml > /tmp/k3s.yml
root@srv11:~# chmod 777 /tmp/k3s.yml 
root@srv11:~# exit
logout
dev01@srv11:~$ ls
srv11
dev01@srv11:~$ mkdir ~/.kube
dev01@srv11:~$ cat /tmp/k3s.yml > ~/.kube/config
dev01@srv11:~$ kubectl get all
WARN[0000] Unable to read /etc/rancher/k3s/k3s.yaml, please start server with --write-kubeconfig-mode to modify kube config permissions 
error: error loading config file "/etc/rancher/k3s/k3s.yaml": open /etc/rancher/k3s/k3s.yaml: permission denied
dev01@srv11:~$ export KUBECONFIG=/home/dev01/.kube/config
dev01@srv11:~$ kubectl get all
NAME                            READY   STATUS    RESTARTS      AGE
pod/cm-acme-http-solver-mshxb   1/1     Running   1 (19d ago)   31d
pod/cm-acme-http-solver-84qms   1/1     Running   1 (19d ago)   31d
pod/app-b-55b46b94bf-2lc7l      1/1     Running   3 (19d ago)   91d
pod/app-a-6868b65647-jqmn9      1/1     Running   3 (19d ago)   91d

NAME                                TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)          AGE
service/kubernetes                  ClusterIP   10.43.0.1       <none>        443/TCP          104d
service/app-a                       ClusterIP   10.43.82.77     <none>        80/TCP           104d
service/app-b                       ClusterIP   10.43.146.156   <none>        80/TCP           103d
service/cm-acme-http-solver-zrlnd   NodePort    10.43.213.123   <none>        8089:30924/TCP   31d
service/cm-acme-http-solver-jt55m   NodePort    10.43.247.48    <none>        8089:30785/TCP   31d

NAME                    READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/app-b   1/1     1            1           91d
deployment.apps/app-a   1/1     1            1           91d

NAME                               DESIRED   CURRENT   READY   AGE
replicaset.apps/app-b-55b46b94bf   1         1         1       91d
replicaset.apps/app-a-6868b65647   1         1         1       91d
dev01@srv11:~$ 
```


```sh
dev01@srv11:~$ flux bootstrap github --token-auth --owner=maximilianou --repository=https://github.com/maximilianou/weekly67/ --branch=main --path=clusters/my-cluster --personal
► connecting to github.com
✗ failed to get Git repository "https://github.com/maximilianou/": provider error: validation error for UserRepositoryRef.RepositoryName: field is required
dev01@srv11:~$ flux bootstrap github --token-auth --owner=maximilianou --repository=weekly67 --branch=main --path=clusters/my-clu
ster --personal
► connecting to github.com
► cloning branch "main" from Git repository "https://github.com/maximilianou/weekly67.git"
✔ cloned repository
► generating component manifests
✔ generated component manifests
✔ committed component manifests to "main" ("7382c8dfb9806a93f49cbe9a790a5e78083942ed")
► pushing component manifests to "https://github.com/maximilianou/weekly67.git"
► installing components in "flux-system" namespace
✔ installed components
✔ reconciled components
► determining if source secret "flux-system/flux-system" exists
► generating source secret
► applying source secret "flux-system/flux-system"
✔ reconciled source secret
► generating sync manifests
✔ generated sync manifests
✔ committed sync manifests to "main" ("833fb6ee38fe70811e38061798365d0c97f3337d")
► pushing sync manifests to "https://github.com/maximilianou/weekly67.git"
► applying sync manifests
✔ reconciled sync configuration
◎ waiting for GitRepository "flux-system/flux-system" to be reconciled
✔ GitRepsoitory reconciled successfully
◎ waiting for Kustomization "flux-system/flux-system" to be reconciled
✔ Kustomization reconciled successfully
► confirming components are healthy
✔ helm-controller: deployment ready
✔ kustomize-controller: deployment ready
✔ notification-controller: deployment ready
✔ source-controller: deployment ready
✔ all components are healthy
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ git pull
remote: Enumerating objects: 14, done.
remote: Counting objects: 100% (14/14), done.
remote: Compressing objects: 100% (8/8), done.
remote: Total 13 (delta 2), reused 11 (delta 0), pack-reused 0
Unpacking objects: 100% (13/13), 47.09 KiB | 405.00 KiB/s, done.
From https://github.com/maximilianou/weekly67
   5d1d334..833fb6e  main       -> origin/main
Updating 5d1d334..833fb6e
Fast-forward
 clusters/my-cluster/flux-system/gotk-components.yaml | 9588 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 clusters/my-cluster/flux-system/gotk-sync.yaml       |   27 +
 clusters/my-cluster/flux-system/kustomization.yaml   |    5 +
 3 files changed, 9620 insertions(+)
 create mode 100644 clusters/my-cluster/flux-system/gotk-components.yaml
 create mode 100644 clusters/my-cluster/flux-system/gotk-sync.yaml
 create mode 100644 clusters/my-cluster/flux-system/kustomization.yaml
```

```sh
dev01@srv11:~$ flux check --pre
► checking prerequisites
✔ Kubernetes 1.27.4+k3s1 >=1.26.0-0
✔ prerequisites checks passed
```


https://github.com/fluxcd/flux2-kustomize-helm-example


```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ mkdir -p apps/base 

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ mkdir -p apps/staging

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ mkdir -p apps/production

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ mkdir -p infrastructure/configs

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ mkdir -p infrastructure/controllers
```

[TODO] https://github.com/fluxcd/flux2-kustomize-helm-example

- Application

```
./apps/
├── base
│   └── podinfo
│       ├── kustomization.yaml
│       ├── namespace.yaml
│       ├── release.yaml
│       └── repository.yaml
├── production
│   ├── kustomization.yaml
│   └── podinfo-patch.yaml
└── staging
    ├── kustomization.yaml
    └── podinfo-patch.yaml
```

```yml
# apps/base/kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
name: podinfo
resources: 
  - namespace.yml
  - repository.yml
  - releases.yml

```

```yml
# apps/base/namespace.yml
apiVersion: v1
kind: namespace
metadata:
  name: podinfo
  labels:
    toolkit.fluxcd.io/tenant: dev-team
```

```yml
# apps/base/repository.yml
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata: 
  name: podinfo
  namespace: podinfo
spec:
  interval: 5m
  url: https://stefanprodan.github.io/podinfo
```

```yml
# apps/base/release.yml
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata:
  name: podinfo
  namespace: podinfo
spec:
  releaseName: podinfo
  chart:
    spec:
      chart: podinfo
      sourceRef:
        kind: HelmRepository
        name: podinfo
  interval: 50m
  install:
    remediation:
      retries: 3
      #Default values 
      # https://github.com/stefanprodan/podinfo/blob/master/charts/podinfo/values.yaml
  values:
    redis:
      enabled: true
      repository: public.ecr.aws/docker/library/redis
      tag: 7.0.0
    ingress:
      enables: true
      className: nginx
```


```yml
# apps/staging/kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
namespace: podinfo
resources:
  - ../base/podinfo
patches:
  - path: podinfo-values.yml
    target:
      kind: HelmRelease
```

```yml
# apps/staging/podinfo-values.yml
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata:
  name: podinfo
  namespace: podinfo
spec:
  chart:
    spec:
      version: ">=1.0.0-alpha"
  test:
    enable:  false
  values:
    ingress:
      hosts:
        - host: podinfo.staging
          paths:
            - path: /
              pathType: ImplementationSpecific
              
```

```yml
# apps/production/kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
resources:
  - ../base/podinfo
patches:
  - path:  podinfo-values.yml
    target:
      kind: HelmRelease
```

```yml
# apps/production/podinfo-values.yml
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata:
  name: podinfo
  namespace: podinfo
spec:
  chart: 
    spec:
      version: ">=1.0.0"
  values:
    ingress:
      hosts: 
        - host: podinfo.production
          paths:
            - path: /
              pathType: ImplementationSpecific  
```

- Infrastrucure

```
./infrastructure/
├── configs
│   ├── cluster-issuers.yaml
│   ├── network-policies.yaml
│   └── kustomization.yaml
└── controllers
    ├── cert-manager.yaml
    ├── ingress-nginx.yaml
    ├── weave-gitops.yaml
    └── kustomization.yaml
```


```yml
# infrastructure/controllers/kustomization.yml
apiVersion: Kustomization
resource:
  - cert-manager.yml
  - ingress-nginx.yml
  - weave-gitops.yml
```

```yml
# infraestructure/controllers/cert-manager.yml
apiVersion: v1
kind: Namespace
metadata: 
  name: cert-manager
  labels: 
    toolkit.fluxcd.io/tenant: sre-team
---
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: cert-manager
  namespace: cert-manager
spec: 
  interval: 24h
  url: https://charts.jetstack.io
---
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata: 
  name: cert-manager
  namespace: cert-manager
spec:
  interval: 30m
  chart:
    spec:
      chart: cert-manager
      version: "1.4"
      sourceRef:
        kind: HelmRepository
        name: cert-manager
        namespace: cert-manager
      interval: 12h
  values:
    installCRDs: true

```

```yml
# infraestructure/controllers/ingress-nginx.yml
apiVersion: v1
kind: Namespace
metadata: 
  name: ingress-nginx
  labels:
    toolkit.fluxcd.io/tenant: sre-team
---
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: ingress-nginx
  namespace: ingress-nginx
spec:
  interval: 24h
  url: https://kubernetes.github.io/ingress-nginx
---
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata:
  name: ingress-nginx
  namespace: ingress-nginx
spec:
  interval: 30m
  chart: 
    spec:
      chart: ingress-nginx
      version: "*"
      sourceRef:
        kind: HelmRepository
        name: ingress-nginx
        namespace: ingress-nginx
      interval: 12h
  values:
    controller:
      service: 
        type: "NodePort"
    admissionWebhooks:
      enabled: false
```

```yml
# infraestructure/controllers/weave-gitops.yml
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: weave-gitops
  namespace: flux-system
spec:
  type: oci
  interval: 60m0s
  url: oci://ghcr.io/weaveworks/charts
---
apiVersion: helm.toolkit.fluxcd.io/v2beta2
kind: HelmRelease
metadata:
  name: weave-gitops
  namespace: flux-system
spec:
  interval: 60m
  chart:
    spec:
      chat: weave-gitops
      version: ""
      sourceRef:
        kind: HelmRepository
        name: weave-gitops
      interval: 12h
    # https://github.com/weaveworks/weave-gitops/blob/main/charts/gitops-server/values.yaml
    values:
      resources:
        requests:
          cpu: 100m
          memory: 64Mi
        limits:
          cpu: 1
          memory: 512Mi
      securityContext:
        capailities:
          drop:
            - ALL
        readOnlyRootFilesystem: true
        runAsNonRoot: true
        runAsUser: 1000
      adminUser:
        create: true
        username: admin
        # Change password by generating a new hash with:
        # https://docs.gitops.weave.works/docs/configuration/securing-access-to-the-dashboard/#login-via-a-cluster-user-account
        # bcrypt hash for password "flux"
        passwordHash: "$2a$10$P/tHQ1DNFXdvX0zRGA8LPeSOyb0JXq9rP3fZ4W8HGTpLV7qHDlWhe"
```


- infrastructure config

```yml
# infrastructure/configs/kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
resources:
  - cluster-issuers.yml
  - network-policies.yml
```

```yml
# infrastructure/configs/cluster-issuers.yml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt
spec:
  acme:
    email: maximilianou@gmail.com
    server: https://acme-staging-v02.api.letsencrypt.org/directory
    privateKeySecretRef:
      name: letsencrypt-nginx
    solvers:
      - http01:
          ingress: 
            class: nginx
```

```yml
# infrastructure/configs/network-policies.yml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: weave-gitops-ingress
  namespace: flux-system
spec:
  policyTypes:
    - Ingress
  ingress:
    - from:
      - namespaceSelector: {}
  podSelector:
    matchLabels:
      app.kubernetes.io/name: weave-gitops
```


```sh
dev01@srv11:~$ flux check --pre
► checking prerequisites
✔ Kubernetes 1.27.4+k3s1 >=1.26.0-0
✔ prerequisites checks passed


dev01@srv11:~$ flux bootstrap github --context=staging --owner=maximilianou --repository=weekly67 --branch=main --personal --path=clusters/staging
Please enter your GitHub personal access token (PAT): 
✗ context "staging" does not exist
```


https://www.airplane.dev/blog/kubectl-config-set-context-tutorial-and-best-practices

- kubernetes Context

kubectl - having no access to remote kubernetes server, will work localy for this flux.
```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ ssh -p 30122 -NL 16443:192.168.1.111:6443 dev01@srv01.simpledoers.com

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl --kubeconfig ~/.kube/config2 get all -A
E1218 09:30:30.350165   38401 memcache.go:265] couldn't get current server API group list: Get "https://localhost:16443/api?timeout=32s": tls: failed to verify certificate: x509: certificate signed by unknown authority
```
----------

- create context with new ( dev-namespace, dev-user )
```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config set-context dev-context --namespace=dev-namespace --cluster=default --user=dev-user
Context "dev-context" created.

┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config view
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: DATA+OMITTED
    server: https://127.0.0.1:6443
  name: default
contexts:
- context:
    cluster: default
    user: default
  name: default
- context:
    cluster: default
    namespace: dev-namespace
    user: dev-user
  name: dev-context
current-context: default
kind: Config
preferences: {}
users:
- name: default
  user:
    client-certificate-data: DATA+OMITTED
    client-key-data: DATA+OMITTED
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config set-context dev-context --namespace=dev-ns --cluster=default --user=dev-usr        
Context "dev-context" modified.
                                                                                                                                                                 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config view                                                                       
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: DATA+OMITTED
    server: https://127.0.0.1:6443
  name: default
contexts:
- context:
    cluster: default
    user: default
  name: default
- context:
    cluster: default
    namespace: dev-ns
    user: dev-usr
  name: dev-context
current-context: default
kind: Config
preferences: {}
users:
- name: default
  user:
    client-certificate-data: DATA+OMITTED
    client-key-data: DATA+OMITTED
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config current-context
default
                                                                                                                                                                 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config set-context dev-context                                                    
Context "dev-context" modified.
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config set-context dev-context
Context "dev-context" modified.
                                                                                                                                                                 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config current-context        
default
```


```
kubectl config set-cluster \
prod \
--server=https://1.2.3.4 \
--certificate-authority=xTsofu101...

kubectl config set-context \
prod \
--namespace=production \
--cluster=prod \
--user=admin

kubectl config set-credentials \
admin \
--client-certificate=<CERTIFICATE> \
--client-key=<KEY>

```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config set-context current --namespace=dev-ns 
Context "current" created.
                                                                                                                                                                 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config current-context                       
default
                                                                                                                                                                 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ kubectl config get-contexts   
CURRENT   NAME          CLUSTER   AUTHINFO   NAMESPACE
          current                            dev-ns
*         default       default   default    
          dev-context   default   dev-usr    dev-ns
```

Reference: the complete step by step with the simplest sample

https://www.digitalocean.com/community/tutorials/how-to-set-up-a-continuous-delivery-pipeline-with-flux-on-digitalocean-kubernetes

https://thenewstack.io/tutorial-a-gitops-deployment-with-flux-on-digitalocean-kubernetes/


```sh
┌──(kali㉿kali)-[~/projects/weekly67/clusters/dev-cluster]
└─$ flux bootstrap github --owner=maximilianou --repository=weekly67 --path=clusters/dev-cluster --personal  
Please enter your GitHub personal access token (PAT): 
► connecting to github.com
► cloning branch "main" from Git repository "https://github.com/maximilianou/weekly67.git"
✔ cloned repository
► generating component manifests
✔ generated component manifests
✔ committed sync manifests to "main" ("0bff9f52c44e25e35a36522fbd9ac23745d8fa1d")
► pushing component manifests to "https://github.com/maximilianou/weekly67.git"
► installing components in "flux-system" namespace
✔ installed components
✔ reconciled components
► determining if source secret "flux-system/flux-system" exists
► generating source secret
✔ public key: ecdsa-sha2----
✔ configured deploy key "flux-system-main-flux-system-./clusters/dev-cluster" for "https://github.com/maximilianou/weekly67"
► applying source secret "flux-system/flux-system"
✔ reconciled source secret
► generating sync manifests
✔ generated sync manifests
✔ committed sync manifests to "main" ("3066ca9e9d81496667b5d41744b47c983feed19a")
► pushing sync manifests to "https://github.com/maximilianou/weekly67.git"
► applying sync manifests
✔ reconciled sync configuration
◎ waiting for Kustomization "flux-system/flux-system" to be reconciled
✔ Kustomization reconciled successfully
► confirming components are healthy
✔ helm-controller: deployment ready
✔ kustomize-controller: deployment ready
✔ notification-controller: deployment ready
✔ source-controller: deployment ready
✔ all components are healthy


┌──(kali㉿kali)-[~/projects/weekly67/clusters/dev-cluster]
└─$ git config --global core.editor vi

┌──(kali㉿kali)-[~/projects/weekly67/clusters/dev-cluster]
└─$ git config pull.rebase false                       
                                                                                                                                                        
┌──(kali㉿kali)-[~/projects/weekly67/clusters/dev-cluster]
└─$ git pull                    
Merge made by the 'ort' strategy.
 clusters/dev-cluster/flux-system/gotk-components.yaml | 8029 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 clusters/dev-cluster/flux-system/gotk-sync.yaml       |   27 +
 clusters/dev-cluster/flux-system/kustomization.yaml   |    5 +
 3 files changed, 8061 insertions(+)
 create mode 100644 clusters/dev-cluster/flux-system/gotk-components.yaml
 create mode 100644 clusters/dev-cluster/flux-system/gotk-sync.yaml
 create mode 100644 clusters/dev-cluster/flux-system/kustomization.yaml

```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get ns -A
NAME              STATUS   AGE
kube-system       Active   25d
default           Active   25d
kube-public       Active   25d
kube-node-lease   Active   25d
metallb-system    Active   25d
ingress-nginx     Active   25d
flux-system       Active   4h14m

```


```sh
flux logs --all-namespaces
...
2023-12-18T14:19:17.036Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:62f471f06abddee5c295a3c51f269d88d7afc26d' 
2023-12-18T14:20:15.946Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:62f471f06abddee5c295a3c51f269d88d7afc26d' 
2023-12-18T14:21:38.436Z info GitRepository/flux-system.flux-system - stored artifact for commit 'wip: add infrastructure.yml ingress-helm-repo..' 
2023-12-18T14:22:36.192Z info GitRepository/flux-system.flux-system - garbage collected 1 artifacts 
2023-12-18T14:22:37.470Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:035e3b55ddaf04c51e1d8d8d8cd62be4d47b6255' 
2023-12-18T14:23:39.030Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:035e3b55ddaf04c51e1d8d8d8cd62be4d47b6255' 
2023-12-18T14:24:42.050Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:035e3b55ddaf04c51e1d8d8d8cd62be4d47b6255' 
2023-12-18T14:25:43.753Z info GitRepository/flux-system.flux-system - no changes since last reconcilation: observed revision 'main@sha1:035e3b55ddaf04c51e1d8d8d8cd62be4d47b6255' 
...
```



--------------------
--------------------

### GitOps - FluxCD Functional New Repository weekly67-1

--------------------
--------------------



```sh 
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ echo $GITHUB_TOKEN | flux bootstrap github --owner=maximilianou --repository=weekly67-1 --path=fleet/default --personal 
Please enter your GitHub personal access token (PAT): 
► connecting to github.com
✔ repository "https://github.com/maximilianou/weekly67-1" created
► cloning branch "main" from Git repository "https://github.com/maximilianou/weekly67-1.git"
✔ cloned repository
...
```


https://github.com/maximilianou/weekly67-1/

https://github.com/maximilianou/weekly67-2/

```sh
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ tree          
.
├── apps
│   ├── ns.yml
│   ├── web-ingress.yml
│   ├── web-service.yml
│   └── web.yml
├── fleet
│   ├── apps.yml
│   ├── default
│   │   ├── flux-system
│   │   │   ├── gotk-components.yaml
│   │   │   ├── gotk-sync.yaml
│   │   │   └── kustomization.yaml
│   │   └── infrastructure.yml
│   └── infrastructure.yml
└── infrastructure
    ├── ingress-helm-release.yml
    ├── ingress-helm-repo.yml
    └── ingress-ns.yml

6 directories, 13 files
```






```sh
┌──(kali㉿kali)-[~]
└─$ flux get sources git                                                                                              
NAME            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     main@sha1:ceb2e2ac      False           True    stored artifact for revision 'main@sha1:ceb2e2ac'


┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ git log --decorate --oneline --graph --all
* ceb2e2a (HEAD -> main, origin/main, origin/HEAD) feat: apps in fleat/default/apps.yml
* e74bf41 feat: apps in fleat/default/apps.yml
* 21e9eed feat: curl http://192.168.106.150/web web.xml
* 818c776 wip: adding infrastructure, apps, ..
* 6c00a68 Add Flux sync manifests
* b29b4aa Add Flux v2.1.2 component manifests

```

Reference: step by step guide to see Logs or status

https://managedkube.com/gitops/flux/weaveworks/guide/tutorial/2020/05/01/a-complete-step-by-step-guide-to-implementing-a-gitops-workflow-with-flux.html


https://fluxcd.io/flux/monitoring/logs/

https://fluxcd.io/flux/cmd/flux_trace/


```sh
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace replicaset.apps/ingress-nginx-controller-6fcf745c45 --namespace ingress-system

Object:         ReplicaSet/ingress-nginx-controller-6fcf745c45
Namespace:      ingress-system
Status:         Managed by Flux
---
HelmRelease:    ingress-nginx
Namespace:      ingress-system
Revision:       
Status:         Last reconciled at 2023-12-18 17:06:00 +0100 CET
Message:        install retries exhausted
---
HelmChart:      ingress-system-ingress-nginx
Namespace:      flux-system
Chart:          ingress-nginx
Version:        4.0.13
Revision:       4.0.13
Status:         Last reconciled at 2023-12-18 17:00:46 +0100 CET
Message:        pulled 'ingress-nginx' chart with version '4.0.13'
---
HelmRepository: ingress-nginx
Namespace:      flux-system
URL:            https://kubernetes.github.io/ingress-nginx
Revision:       sha256:39523c
Status:         Last reconciled at 2023-12-18 17:00:45 +0100 CET
Message:        stored artifact: revision 'sha256:39523c'


┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace namespace mywebapp                                                            

Object:          Namespace/mywebapp
Status:          Managed by Flux
---
Kustomization:   app
Namespace:       flux-system
Path:            ./apps
Revision:        
Status:          Last reconciled at 2023-12-19 11:28:07 +0100 CET
Message:         Ingress/mywebapp/mywebapp-ingress dry-run failed: failed to create typed patch object (mywebapp/mywebapp-ingress; networking.k8s.io/v1, Kind=Ingress): .spec.rules[0].paths: field not declared in schema

---
GitRepository:   flux-system
Namespace:       flux-system
URL:             ssh://git@github.com/maximilianou/weekly67-1
Branch:          main
Revision:        main@sha1:ceb2e2acc667c463bfc38559bbb966e2652a25dd
Status:          Last reconciled at 2023-12-19 09:00:46 +0100 CET
Message:         stored artifact for revision 'main@sha1:ceb2e2acc667c463bfc38559bbb966e2652a25dd'




┌──(kali㉿kali)-[~]
└─$ flux get sources git     
NAME            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     main@sha1:ceb2e2ac      False           True    stored artifact for revision 'main@sha1:ceb2e2ac'

┌──(kali㉿kali)-[~]
└─$ flux reconcile source git flux-system
► annotating GitRepository flux-system in flux-system namespace
✔ GitRepository annotated
◎ waiting for GitRepository reconciliation
✔ fetched revision main@sha1:ceb2e2acc667c463bfc38559bbb966e2652a25dd



Message:         Ingress/mywebapp/mywebapp-ingress dry-run failed: failed to create typed patch object (mywebapp/mywebapp-ingress; networking.k8s.io/v1, Kind=Ingress): .spec.rules[0].paths: field not declared in schema
```


```yml
# apps/web-ingress.yml  
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: mywebapp-ingress
  namespace: mywebapp
#  annotations:
#    kubernetes.io/ingress.class: nginx
spec:
  ingressClassName: nginx
  rules:
  - http:
      paths:
      - path: /web
        pathType: Prefix
        backend:
          service:
            name: web
            port:
              number: 80
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace namespace mywebapp

Object:          Namespace/mywebapp
Status:          Managed by Flux
---
Kustomization:   app
Namespace:       flux-system
Path:            ./apps
Revision:        
Status:          Last reconciled at 2023-12-19 12:35:17 +0100 CET
Message:         Ingress/mywebapp/mywebapp-ingress dry-run failed, reason: InternalError: Internal error occurred: failed calling webhook "validate.nginx.ingress.kubernetes.io": failed to call webhook: Post "https://ingress-nginx-controller-admission.ingress-system.svc:443/networking/v1/ingresses?timeout=10s": tls: failed to verify certificate: x509: certificate signed by unknown authority

---
GitRepository:   flux-system
Namespace:       flux-system
URL:             ssh://git@github.com/maximilianou/weekly67-1
Branch:          main
Revision:        main@sha1:f1f1bac07db4cf899c9b667c308357c24f3dabef
Status:          Last reconciled at 2023-12-19 12:35:16 +0100 CET
Message:         stored artifact for revision 'main@sha1:f1f1bac07db4cf899c9b667c308357c24f3dabef'

```


```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all --namespace mywebapp
No resources found in mywebapp namespace.

```

> Reference:
> tls: failed to verify certificate: x509: certificate signed by unknown authority
> https://github.com/kubernetes/ingress-nginx/issues/5968

```sh

┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ CA=$(kubectl -n ingress-system get secret ingress-nginx-admission -ojsonpath='{.data.ca}')
kubectl patch validatingwebhookconfigurations ingress-nginx-admission --type='json' -p='[{"op": "add", "path": "/webhooks/0/clientConfig/caBundle", "value":"'$CA'"}]'

```

```sh
git add .
git commit -m "fix: bug reference https://github.com/kubernetes/ingress-nginx/issues/5968 "
git push
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace namespace mywebapp

Object:          Namespace/mywebapp
Status:          Managed by Flux
---
Kustomization:   app
Namespace:       flux-system
Path:            ./apps
Revision:        main@sha1:bc5ab95e4c13af3cdb0b7b72decddd558961d458
Status:          Last reconciled at 2023-12-19 12:48:20 +0100 CET
Message:         Applied revision: main@sha1:bc5ab95e4c13af3cdb0b7b72decddd558961d458
---
GitRepository:   flux-system
Namespace:       flux-system
URL:             ssh://git@github.com/maximilianou/weekly67-1
Branch:          main
Revision:        main@sha1:bc5ab95e4c13af3cdb0b7b72decddd558961d458
Status:          Last reconciled at 2023-12-19 12:45:31 +0100 CET
Message:         stored artifact for revision 'main@sha1:bc5ab95e4c13af3cdb0b7b72decddd558961d458'

```


```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all --namespace mywebapp
NAME                       READY   STATUS    RESTARTS   AGE
pod/web-798dd4ffc6-s4vlh   1/1     Running   0          6m1s
pod/web-798dd4ffc6-pwthg   1/1     Running   0          6m1s
pod/web-798dd4ffc6-gx8bf   1/1     Running   0          6m1s

NAME          TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)   AGE
service/web   ClusterIP   10.43.117.91   <none>        80/TCP    6m1s

NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/web   3/3     3            3           6m1s

NAME                             DESIRED   CURRENT   READY   AGE
replicaset.apps/web-798dd4ffc6   3         3         3       6m1s

```


```sh
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace namespace ingress-system

Object:          Namespace/ingress-system
Status:          Managed by Flux
---
Kustomization:   infrastructure
Namespace:       flux-system
Path:            ./infrastructure
Revision:        main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
Status:          Last reconciled at 2023-12-19 14:50:24 +0100 CET
Message:         Applied revision: main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
---
GitRepository:   flux-system
Namespace:       flux-system
URL:             ssh://git@github.com/maximilianou/weekly67-1
Branch:          main
Revision:        main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
Status:          Last reconciled at 2023-12-19 12:48:42 +0100 CET
Message:         stored artifact for revision 'main@sha1:f0a6f108950e642f1752670b50884802cf8563b6'
                                                                                                                                                                                             
┌──(kali㉿kali)-[~/projects/weekly67-1]
└─$ flux trace namespace flux-system   

Object:          Namespace/flux-system
Status:          Managed by Flux
---
Kustomization:   flux-system
Namespace:       flux-system
Path:            ./fleet/default
Revision:        main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
Status:          Last reconciled at 2023-12-19 14:44:35 +0100 CET
Message:         Applied revision: main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
---
GitRepository:   flux-system
Namespace:       flux-system
URL:             ssh://git@github.com/maximilianou/weekly67-1
Branch:          main
Revision:        main@sha1:f0a6f108950e642f1752670b50884802cf8563b6
Status:          Last reconciled at 2023-12-19 12:48:42 +0100 CET
Message:         stored artifact for revision 'main@sha1:f0a6f108950e642f1752670b50884802cf8563b6'
                                                                                                       
```


----------
----------

### weekly67-2

- kubernetes

```sh
┌──(kali㉿kali)-[~]
└─$ /usr/local/bin/k3s-uninstall.sh
```


```sh
┌──(kali㉿kali)-[~]
└─$ ps aux | grep k3s
kali       23673  0.0  0.0   6344  2176 pts/0    S+   09:56   0:00 grep --color=auto k3s

```

```sh
┌──(kali㉿kali)-[~]
└─$ curl -sfL https://get.k3s.io | sh -                           
[INFO]  Installing k3s to /usr/local/bin/k3s
[INFO]  Creating /usr/local/bin/kubectl symlink to k3s
[INFO]  Creating /usr/local/bin/crictl symlink to k3s
[INFO]  Skipping /usr/local/bin/ctr symlink to k3s, command exists in PATH at /usr/bin/ctr
[INFO]  Creating killall script /usr/local/bin/k3s-killall.sh
[INFO]  Creating uninstall script /usr/local/bin/k3s-uninstall.sh
[INFO]  env: Creating environment file /etc/systemd/system/k3s.service.env
[INFO]  systemd: Creating service file /etc/systemd/system/k3s.service
Created symlink /etc/systemd/system/multi-user.target.wants/k3s.service → /etc/systemd/system/k3s.service.
[INFO]  systemd: Starting k3s

```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all -A                          
E1221 10:07:25.717654   32023 memcache.go:265] couldn't get current server API group list: Get "https://127.0.0.1:6443/api?timeout=32s": tls: failed to verify certificate: x509: certificate signed by unknown authority
```

```sh
┌──(kali㉿kali)-[~]
└─$ sudo cat /etc/rancher/k3s/k3s.yaml > ~/.kube/config
```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all -A
NAMESPACE     NAME                                          READY   STATUS      RESTARTS   AGE
kube-system   pod/local-path-provisioner-84db5d44d9-rcghq   1/1     Running     0          6m46s
```

- FluxCD

```sh
┌──(kali㉿kali)-[~]
└─$ echo $GITHUB_TOKEN | flux bootstrap github --owner=maximilianou --repository=weekly67-2 --path=kube01/default --personal
✔ all components are healthy
```

```sh
┌──(kali㉿kali)-[~/projects]
└─$ git clone https://github.com/maximilianou/weekly67-2
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ tree       
.
└── kube01
    └── default
        └── flux-system
            ├── gotk-components.yaml
            ├── gotk-sync.yaml
            └── kustomization.yaml
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ mkdir apps  

┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ mkdir infrastructura
```

```sh
┌──(kali㉿kali)-[~]
└─$ flux get sources git
NAME            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     main@sha1:3302ab5e      False           True    stored artifact for revision 'main@sha1:3302ab5e'
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ tree                                   
.
├── apps
├── infrastructure
└── kube01
    └── default
        ├── apps.yml
        ├── flux-system
        │   ├── gotk-components.yaml
        │   ├── gotk-sync.yaml
        │   └── kustomization.yaml
        └── infrascructure.yml
```

```yml
# kube01/default/apps.yml
apiVersion: kustomize.toolkit.fluxcd.io/v1beta1
kind: Kustomization
metadata:
  name: app
  namespace: flux-system
spec:
  interval: 1m
  sourceRef:
    kind: GitRepository
    name: flux-system
  path: ./apps
  prune: true
```

```yml
# kube01/default/infrastructure.yml
apiVersion: kustomize.toolkit.fluxcd.io/v1beta1
kind: Kustomization
metadata:
  name: infrastructure
  namespace: flux-system
spec:
  interval: 1m
  sourceRef:
    kind: GitRepository
    name: flux-system
  path: ./infrastructure
  prune: true
```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all --namespace flux-system
NAME                                          READY   STATUS    RESTARTS   AGE
pod/source-controller-6c49485888-726cv        1/1     Running   0          60m
pod/notification-controller-76dc5d768-x7jcd   1/1     Running   0          60m
pod/helm-controller-5f964c6579-8w4s9          1/1     Running   0          60m
pod/kustomize-controller-9c588946c-wbgc7      1/1     Running   0          60m

NAME                              TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)   AGE
service/notification-controller   ClusterIP   10.43.130.187   <none>        80/TCP    60m
service/source-controller         ClusterIP   10.43.236.92    <none>        80/TCP    60m
service/webhook-receiver          ClusterIP   10.43.94.126    <none>        80/TCP    60m

NAME                                      READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/helm-controller           1/1     1            1           60m
deployment.apps/kustomize-controller      1/1     1            1           60m
deployment.apps/notification-controller   1/1     1            1           60m
deployment.apps/source-controller         1/1     1            1           60m

NAME                                                DESIRED   CURRENT   READY   AGE
replicaset.apps/source-controller-6c49485888        1         1         1       60m
replicaset.apps/notification-controller-76dc5d768   1         1         1       60m
replicaset.apps/helm-controller-5f964c6579          1         1         1       60m
replicaset.apps/kustomize-controller-9c588946c      1         1         1       60m
```

- metallb


```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ curl https://raw.githubusercontent.com/metallb/metallb/v0.13.12/config/manifests/metallb-native.yaml > infrastructure/metallb/metallb-native.yml
```

```yml
# infra/metallb-ipaddr.yml
apiVersion: metallb.io/v1beta1
kind: IPAddressPool
metadata:
  name: first-pool
  namespace: metallb-system
spec:
  addresses:
  - 192.168.106.150-192.168.106.170
```

```yml
# infra/metallb-L2adv.yml
apiVersion: metallb.io/v1beta1
kind: L2Advertisement
metadata:
  name: metallb-l2adv
  namespace: metallb-system
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ cat infrastructure/metallb/metallb-native.yml | grep namespace
          namespace: metallb-system
```

```sh
┌──(kali㉿kali)-[~]
└─$ flux trace namespace metallb-system
✗ x: namespaces "metallb-system" not found

```

```sh
┌──(kali㉿kali)-[~]
└─$ flux tree kustomization flux-system --compact
Kustomization/flux-system/flux-system
└── GitRepository/flux-system/flux-system

```


- Reconcile FluxCD system
```sh
┌──(kali㉿kali)-[~]
└─$ flux reconcile source git flux-system        
► annotating GitRepository flux-system in flux-system namespace
✔ GitRepository annotated
◎ waiting for GitRepository reconciliation
✔ fetched revision main@sha1:94c7fdb14d9f005d88ce8f848fdeaf79012f694f

```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get ns    
NAME              STATUS   AGE
kube-system       Active   113m
kube-public       Active   113m
kube-node-lease   Active   113m
default           Active   113m
flux-system       Active   104m
ingress-system    Active   79s
metallb-system    Active   79s

```


```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all --namespace metallb-system
NAME                              READY   STATUS    RESTARTS   AGE
pod/controller-786f9df989-8nv2q   1/1     Running   0          2m20s
pod/speaker-fkdhq                 1/1     Running   0          2m20s

NAME                      TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)   AGE
service/webhook-service   ClusterIP   10.43.201.131   <none>        443/TCP   2m20s

NAME                     DESIRED   CURRENT   READY   UP-TO-DATE   AVAILABLE   NODE SELECTOR            AGE
daemonset.apps/speaker   1         1         1       1            1           kubernetes.io/os=linux   2m20s

NAME                         READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/controller   1/1     1            1           2m20s

NAME                                 W   DESIRED   CURRENT   READY   AGE
replicaset.apps/controller-786f9df989   1         1         1       2m20s

```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get svc -A
NAMESPACE        NAME                      TYPE           CLUSTER-IP      EXTERNAL-IP       PORT(S)                      AGE
default          kubernetes                ClusterIP      10.43.0.1       <none>            443/TCP                      117m
kube-system      kube-dns                  ClusterIP      10.43.0.10      <none>            53/UDP,53/TCP,9153/TCP       117m
kube-system      metrics-server            ClusterIP      10.43.52.13     <none>            443/TCP                      117m
flux-system      notification-controller   ClusterIP      10.43.130.187   <none>            80/TCP                       107m
flux-system      source-controller         ClusterIP      10.43.236.92    <none>            80/TCP                       107m
flux-system      webhook-receiver          ClusterIP      10.43.94.126    <none>            80/TCP                       107m
metallb-system   webhook-service           ClusterIP      10.43.201.131   <none>            443/TCP                      4m45s
kube-system      traefik                   LoadBalancer   10.43.46.34     192.168.106.150   80:30627/TCP,443:30956/TCP   116m

```


- Reconcile FluxCD source git flus-system
```sh
┌──(kali㉿kali)-[~]
└─$ flux reconcile source git flux-system
► annotating GitRepository flux-system in flux-system namespace
✔ GitRepository annotated
◎ waiting for GitRepository reconciliation
✔ fetched revision main@sha1:996d7e60df3628f9e4bf02d7184bb1da1eba7baf

```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get all --namespace ingress-system
NAME                                            READY   STATUS      RESTARTS   AGE
pod/ingress-nginx-controller-6fcf745c45-pbm9k   1/1     Running     0          35s
pod/ingress-nginx-admission-patch-fxxbh         0/1     Completed   0          4s

NAME                                         TYPE           CLUSTER-IP     EXTERNAL-IP       PORT(S)                      AGE
service/ingress-nginx-controller-admission   ClusterIP      10.43.56.8     <none>            443/TCP                      35s
service/ingress-nginx-controller             LoadBalancer   10.43.68.236   192.168.106.151   80:31916/TCP,443:31424/TCP   35s

NAME                                       READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/ingress-nginx-controller   1/1     1            1           35s

NAME                                                  DESIRED   CURRENT   READY   AGE
replicaset.apps/ingress-nginx-controller-6fcf745c45   1         1         1       35s

NAME                                      COMPLETIONS   DURATION   AGE
job.batch/ingress-nginx-admission-patch   0/1           4s         4s

```

- test reconcile automatic over time

```yml
# infra/ns/ns.yml
apiVersion: v1
kind: Namespace
metadata:
  name: namespace-test-system
```

```sh
git add .
git commit 
git push
```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get ns                            
NAME                    STATUS   AGE
namespace-test-system   Active   3m46s
```

- Commenting the content didn't undeploy with FluxCD automatic reconcile
```yml
# infra/ns/ns.yml
#apiVersion: v1
#kind: Namespace
#metadata:
#  name: namespace-test-system
```

```sh
git add
git commit
git push
```

```sh
┌──(kali㉿kali)-[~]
└─$ kubectl get ns
namespace-test-system   Active   8m2s
```

- Delete the file to Undeploy with FluxCD automatic reconcile
```sh
┌──(kali㉿kali)-[~/projects]
└─$ rm weekly67-2/infra/ns/ns.yml
```

```sh
┌──(kali㉿kali)-[~/projects]
└─$ kubectl get ns
```


----------



```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ flux get all -A     
NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     gitrepository/flux-system       main@sha1:2fd679b0      False           True    stored artifact for revision 'main@sha1:2fd679b0'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                                     
flux-system     helmrepository/ingress-nginx    sha256:e6a6c9e8 False           True    stored artifact: revision 'sha256:e6a6c9e8'

NAMESPACE       NAME                                    REVISION        SUSPENDED       READY   MESSAGE                                            
flux-system     helmchart/ingress-system-ingress-nginx  4.0.13          False           True    pulled 'ingress-nginx' chart with version '4.0.13'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                          
ingress-system  helmrelease/ingress-nginx       4.0.13          False           True    Release reconciliation succeeded

NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                                                                                    
flux-system     kustomization/flux-system       main@sha1:2fd679b0      False           True    Applied revision: main@sha1:2fd679b0                                                      
flux-system     kustomization/infra             main@sha1:2fd679b0      False           True    Applied revision: main@sha1:2fd679b0                                                      
flux-system     kustomization/apps                                      False           False   Deployment/app-a namespace not specified: the server could not find the requested resource

```



------

- apps.yml
- apps/ans/app-a-ns.yml
```yml
# apps/app-ns.yml
apiVersion: v1
kind: Namespace
metadata:
  name: app-a
```
- apps/ans/app-a.yml
```yml
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: app-a
  namespace: app-a
  labels:
    app: app-a
spec:
  replicas: 1
  selector:
    matchLabels:
      app: app-a
  template:
    metadata:
      labels:
        app: app-a
    spec:
      containers:
      - name: nginx
        image: nginx
        ports:
        - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: app-a
  namespace: app-a
spec:
  selector:
    app: app-a
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80
---
```
- apps/ans/app-a-ingress.yml
```yml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: ingress-resource
  namespace: app-a
#  annotations:
#    cert-manager.io/cluster-issuer: letsencrypt-staging
spec:
  ingressClassName: nginx
#  tls:
#  - hosts:
#    - a.kube.simpledoers.work
#    secretName: letsencrypt-staging
  rules:
  - host: a.kube
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: app-a
            port:
              number: 80
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ flux reconcile source git flux-system
► annotating GitRepository flux-system in flux-system namespace
✔ GitRepository annotated
◎ waiting for GitRepository reconciliation
✔ fetched revision main@sha1:b04b3757459a15cfa2f8453058bf554d1af0e502


┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ flux get all -A                      
NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     gitrepository/flux-system       main@sha1:b04b3757      False           True    stored artifact for revision 'main@sha1:b04b3757'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                                     
flux-system     helmrepository/ingress-nginx    sha256:e6a6c9e8 False           True    stored artifact: revision 'sha256:e6a6c9e8'

NAMESPACE       NAME                                    REVISION        SUSPENDED       READY   MESSAGE                                            
flux-system     helmchart/ingress-system-ingress-nginx  4.0.13          False           True    pulled 'ingress-nginx' chart with version '4.0.13'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                          
ingress-system  helmrelease/ingress-nginx       4.0.13          False           True    Release reconciliation succeeded

NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                              
flux-system     kustomization/apps              main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757
flux-system     kustomization/infra             main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757
flux-system     kustomization/flux-system       main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757


┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ curl http://a.kube/
<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
html { color-scheme: light dark; }
body { width: 35em; margin: 0 auto;
font-family: Tahoma, Verdana, Arial, sans-serif; }
</style>
</head>
<body>
<h1>Welcome to nginx!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>
<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>
<p><em>Thank you for using nginx.</em></p>
</body>
</html>

```
https://fluxcd.io/flux/cheatsheets/troubleshooting/

https://docs.gitlab.com/ee/user/packages/container_registry/build_and_push_images.html


Reference: That DevOps Guy

Github - Drone - Github action running into kubernetes.

https://www.youtube.com/watch?v=myCcJJ_Fk10


----------

https://github.com/marcel-dempers/docker-development-youtube-series/tree/master/drone-ci


```yml
# infra/drone/drone-ns.yml
apiVersion: v1
kind: Namespace
metadata:
  name: drone
```

```yml
# infra/drone/drone-pgsql.yml
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: postgres-config
  namespace: drone
  labels:
    app: postgres
data:
  POSTGRES_DB: postgresdb
  POSTGRES_USER: postgresadmin
  POSTGRES_PASSWORD: admin123
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
  namespace: drone
spec:
  serviceName: "postgres"
  selector:
    matchLabels:
      app: postgres
  replicas: 1
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
        - name: postgres
          image: postgres:10.4
          imagePullPolicy: "IfNotPresent"
          ports:
            - containerPort: 5432
          envFrom:
            - configMapRef:
                name: postgres-config
---
apiVersion: v1
kind: Service
metadata:
  name: postgres
  namespace: drone
  labels:
    app: postgres
spec:
  selector:
    app: postgres
  ports:
    - protocol: TCP
      name: http
      port: 5432
      targetPort: 5432
```


```sh
┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ flux get all -A    
NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                                           
flux-system     gitrepository/flux-system       main@sha1:b04b3757      False           True    stored artifact for revision 'main@sha1:b04b3757'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                                     
flux-system     helmrepository/ingress-nginx    sha256:e6a6c9e8 False           True    stored artifact: revision 'sha256:e6a6c9e8'

NAMESPACE       NAME                                    REVISION        SUSPENDED       READY   MESSAGE                                            
flux-system     helmchart/ingress-system-ingress-nginx  4.0.13          False           True    pulled 'ingress-nginx' chart with version '4.0.13'

NAMESPACE       NAME                            REVISION        SUSPENDED       READY   MESSAGE                          
ingress-system  helmrelease/ingress-nginx       4.0.13          False           True    Release reconciliation succeeded

NAMESPACE       NAME                            REVISION                SUSPENDED       READY   MESSAGE                              
flux-system     kustomization/flux-system       main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757
flux-system     kustomization/apps              main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757
flux-system     kustomization/infra             main@sha1:b04b3757      False           True    Applied revision: main@sha1:b04b3757


┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ kubectl get ns    
NAME              STATUS   AGE
kube-system       Active   29h
kube-public       Active   29h
kube-node-lease   Active   29h
default           Active   29h
flux-system       Active   29h
ingress-system    Active   27h
metallb-system    Active   27h
ns-test-system    Active   53m
app-a             Active   16m
drone             Active   10s


┌──(kali㉿kali)-[~/projects/weekly67-2]
└─$ kubectl get all --namespace drone
NAME             READY   STATUS    RESTARTS   AGE
pod/postgres-0   1/1     Running   0          41s

NAME               TYPE        CLUSTER-IP    EXTERNAL-IP   PORT(S)    AGE
service/postgres   ClusterIP   10.43.106.9   <none>        5432/TCP   42s

NAME                        READY   AGE
statefulset.apps/postgres   1/1     42s

```



```yml
# infra/drone/droneserver-dpl.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: drone-server
  namespace: drone
  labels:
    app: drone-server
  annotations:
spec:
  selector:
    matchLabels:
      app: drone-server
  replicas: 1
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  template:
    metadata:
      labels:
        app: drone-server
    spec:
      containers:
      - name: drone-server
        image: drone/drone:2
        imagePullPolicy: IfNotPresent
        ports:
        - containerPort: 80
        - containerPort: 443
        env:
        - name: DRONE_USER_CREATE 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_USER_CREATE
        - name: DRONE_DATABASE_DRIVER
          value: postgres
        - name: DRONE_DATABASE_DATASOURCE 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_DATABASE_DATASOURCE
        - name: DRONE_SERVER_PROTO
          value: https
        - name: DRONE_SERVER_HOST 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_SERVER_HOST
        - name: DRONE_GITHUB_CLIENT_ID 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_GITHUB_CLIENT_ID
        - name: DRONE_GITHUB_CLIENT_SECRET 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_GITHUB_CLIENT_SECRET
        - name: DRONE_RPC_SECRET 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_RPC_SECRET
```

```yml
# infra/drone/droneserver-ing.yml
apiVersion: extensions/v1beta1
kind: Ingress
metadata:
  name: drone-server
  namespace: drone
spec:
  ingressClassName: nginx
  rules:
  - host: drone.kube1
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: drone-server
            port:
              number: 80

```


```yml
# infra/drone/droneserver-secret.yml
apiVersion: v1
kind: Secret
metadata:
  name: drone-server-secret
type: Opaque
data:
  DRONE_GITHUB_CLIENT_ID: xxxxxxx     #Get this from GitHub OAUTH
  DRONE_GITHUB_CLIENT_SECRET: xxxxxxx #Get this from GitHub OAUTH
  DRONE_RPC_SECRET: xxxxxxx           #openssl rand -hex 16
  DRONE_DATABASE_DATASOURCE: xxxxxxx  #postgres://postgresadmin:admin123@postgres:5432/postgresdb?sslmode=disable
  DRONE_USER_CREATE: xxxxxxx          #username:marcel-dempers,admin:true
  DRONE_SERVER_HOST: xxxxxxx          #drone.marceldempers.dev
```

```yml
# infra/drone/droneserver-srv.yml
apiVersion: v1
kind: Service
metadata:
  name: drone-server
  namespace: drone
  labels:
    app: drone-server
spec:
  type: ClusterIP
  selector:
    app: drone-server
  ports:
    - protocol: TCP
      name: http
      port: 80
      targetPort: 80
    - protocol: TCP
      name: https
      port: 443
      targetPort: 443
```

```yml
# infra/drone/dronerunner-rbac.yml
kind: Role
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  namespace: drone
  name: drone-runner
rules:
- apiGroups:
  - ""
  resources:
  - secrets
  verbs:
  - create
  - delete
- apiGroups:
  - ""
  resources:
  - pods
  - pods/log
  verbs:
  - get
  - create
  - delete
  - list
  - watch
  - update
---
kind: RoleBinding
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: drone-runner
  namespace: drone
subjects:
- kind: ServiceAccount
  name: drone-runner
  namespace: drone
roleRef:
  kind: Role
  name: drone-runner
  apiGroup: rbac.authorization.k8s.io
```

```yml
# infra/drone/dronerunner.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: drone-runner
  namespace: drone
  labels:
    app.kubernetes.io/name: drone-runner
spec:
  replicas: 1
  selector:
    matchLabels:
      app.kubernetes.io/name: drone
  template:
    metadata:
      labels:
        app.kubernetes.io/name: drone
    spec:
      serviceAccountName: drone-runner
      containers:
      - name: runner
        image: drone/drone-runner-kube:latest
        ports:
        - containerPort: 3000
        env:
        - name: DRONE_NAMESPACE_DEFAULT
          value: drone
        - name: DRONE_SERVICE_ACCOUNT_DEFAULT
          value: drone-runner
        - name: DRONE_RPC_HOST
          value: droneserver.drone
        - name: DRONE_RPC_PROTO
          value: http
        - name: DRONE_RPC_SECRET 
          valueFrom:
            secretKeyRef:
              name: drone-server-secret
              key: DRONE_RPC_SECRET
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: drone-runner
  namespace: drone
  labels:
    app.kubernetes.io/name: drone-runner
```


```yml

---
kind: pipeline
type: kubernetes
name: default

steps:
- name: build-push
  image: docker:dind
  volumes:
  - name: dockersock
    path: /var/run
  environment:
    DOCKER_USER:
      from_secret: DOCKER_USER
    DOCKER_PASSWORD:
      from_secret: DOCKER_PASSWORD
  commands:
  - sleep 5 ## give docker enough time to start
  - docker login -u $DOCKER_USER -p $DOCKER_PASSWORD
  - docker build ./golang -t aimvector/golang:1.0.0
  - docker push aimvector/golang:1.0.0

services:
- name: docker
  image: docker:dind
  privileged: true
  volumes:
  - name: dockersock
    path: /var/run
volumes:
- name: dockersock
  temp: {}
```


----------
----------


- Github Action

```yml
# .github/workflows/nodejs-docker.yml
name: Create and publish a Docker image to GitHub Container Registry

on:
  push:
    branches: ['main']

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build-and-push-image:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v3

      - name: Log in to the Container registry
        uses: docker/login-action@v2
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract metadata (tags, labels) for Docker
        id: meta
        uses: docker/metadata-action@v4
        with:
          #images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
      - name: Build and push Docker image
        uses: docker/build-push-action@v4
        with:
          context: devjs/front1/front1/.
          push: true
          # tags: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest
          labels: ${{ steps.meta.outputs.labels }}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly67]
└─$ docker run ghcr.io/maximilianou/weekly67

Unable to find image 'ghcr.io/maximilianou/weekly67:latest' locally
latest: Pulling from maximilianou/weekly67
661ff4d9561e: Already exists 
d3ae11a87360: Already exists 
...
```

https://gitlab.com/maximilianou/weekly67plan

- yew - rust - webassembly - like react

https://yew.rs/docs/getting-started/build-a-sample-app

https://github.com/jetli/awesome-yew?tab=readme-ov-file

https://github.com/rust-unofficial/awesome-rust



```
```

----------
----------
