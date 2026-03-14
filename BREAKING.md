# 破坏性更新与更新说明

此文件用于说明各版本中 Python 侧 API 的变动

---

## 目录

- [破坏性更新与更新说明](#破坏性更新与更新说明)
  - [目录](#目录)
  - [详细信息](#详细信息)
    - [v0.0.1](#v001)
    - [v0.0.1a0](#v001a0)
    - [v0.0.2](#v002)

---

## 详细信息

### v0.0.1

此版本是项目的初始版本, 不存在破坏性更新

---

### v0.0.1a0

此版本实际上是初始版本的补丁, 处理了 readme 中的部分问题, 只是版本号写错了

---

### v0.0.2

| 原 API                        | 新 API                      |
|------------------------------|----------------------------|
| `Listener.diff_full`         | `Differ.diff_full`         |
| `Listener.diff_without_time` | `Differ.diff_without_time` |
| `Listener.diff_by_id`        | `Differ.diff_by_id`        |
| `Listener.serialize`         | `Differ.serialize`         |

---
