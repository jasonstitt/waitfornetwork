# `waitfornetwork`: an init container for Kubernetes jobs

Kubernetes pods have a little-known race condition. The network routes for the pod are not guaranteed to exist by the time the main process in a container starts running. This is almost never a problem for long-running server processes, but can break jobs and other processes that start up quickly and expect to make a network request immediately upon startup.

`waitfornetwork` is a micro-utility that you put in an `initContainer` to stop your main container from starting until the network is available.

## 🚀 Command-line usage

`--host <ip|hostname>` - host to which to attempt a connection. This could be a private ClusterIP or a public host. Within a Kubernetes cluster, you may wish to choose the IP address of your DNS service as this is both cluster-internal (therefore within your control) and essential. Defaults to `8.8.8.8` which is a public Google DNS server.

`--port <port>` - port on which to attempt a connection. Defaults to 53 (DNS).

`--timeout <seconds>` - timeout after which to fail the process (with exit code 1). Defaults to 60 (seconds).

`--interval <seconds>` - interval at which to run the check. Defaults to 5 (seconds).

## 💡 Example `initContainer`

To use `waitfornetwork` in Kubernetes, you'll want to run it as an `initContainer` so that it blocks the startup of the main containers until successful. Here's an example using a job:

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: example-waitfornetwork
spec:
  template:
    spec:
      containers:
      initContainers:
      - name: waitfornetwork
        image: waitfornetwork:latest
        args: ["--host", "10.96.0.10"]
      - name: main
        image: ubuntu
        command: ["sh", "-c", "echo Hello, network!"]
      restartPolicy: Never
```
