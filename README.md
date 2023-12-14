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





