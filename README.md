# `waitfornetwork`: an init container for Kubernetes jobs

Kubernetes pods have a little-known race condition. The network routes for the pod are not guaranteed to exist by the time the main process in a container starts running. This is almost never a problem for long-running server processes, but can break jobs and other processes that start up quickly and expect to make a network request immediately upon startup.

`waitfornetwork` is a micro-utility that you put in an `initContainer` to stop your main container from starting until the network is available.

## Command-line usage

foo

## Example `initContainer`

foo

```yaml
type: foo
```
