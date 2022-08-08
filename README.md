# Notify Bridge

## Supported Alert Applications

- [Grafana](http://grafana.com/)
- [Alicloud Monitor](https://www.aliyun.com/product/jiankong)

## Supported Notify Applications

- [Feishu(Lark)](https://open.larksuite.com/document/ukTMukTMukTM/uczM3QjL3MzN04yNzcDN)

## Docker Image

### Build

```bash
# DOCKER_BUILDKIT=1 docker build --tag notify-bridge .
# DOCKER_BUILDKIT=1 docker build --build-arg BUILDER_SUFFIX=disable-mirror --tag notify-bridge .
DOCKER_BUILDKIT=1 docker build --tag notify-bridge .
```

### Run

```bash
docker run -it --rm -p 3030:3030 -e RUST_LOG=info notify-bridge
```
