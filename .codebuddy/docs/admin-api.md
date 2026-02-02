# Palpo Admin API 完整文档

基于对 `/Users/tmacy/palpo/crates/server/src/routing/admin/` 目录的深入分析。

## 用户管理模块

### 用户详情
**GET /_synapse/admin/v2/users/{user_id}**
- 功能：获取单个用户的详细信息
- 认证：需要管理员权限（Bearer token）
- 路径参数：user_id（用户 ID，如 @user:server.com）
- 响应：包含用户信息、ThreePIDs、外部 IDs、状态等

### 创建/修改用户
**PUT /_synapse/admin/v2/users/{user_id}**
- 功能：创建新用户或修改现有用户信息
- 认证：需要管理员权限（Bearer token）
- 请求体：密码、显示名、头像、ThreePIDs、外部 IDs、管理员状态等
- 响应：更新后的用户信息

### 用户列表
**GET /_synapse/admin/v2/users**
- 功能：获取用户列表（分页、过滤、排序）
- 查询参数：from, limit, user_id, name, guests, deactivated, admins, order_by, dir
- 响应：用户数组、总数、下一页令牌

**GET /_synapse/admin/v3/users**
- 功能：v3 版本用户列表（类似 v2）

### 用户操作
**POST /_synapse/admin/v1/deactivate/{user_id}**
- 功能：停用用户账户
- 请求体：{ "erase": false }

**POST /_synapse/admin/v1/reset_password/{user_id}**
- 功能：重置用户密码
- 请求体：{ "new_password": "...", "logout_devices": true }

**PUT /_synapse/admin/v1/users/{user_id}/admin**
- 功能：设置用户的管理员状态
- 请求体：{ "admin": true }

**POST /_synapse/admin/v1/users/{user_id}/shadow_ban**
- 功能：影子封禁用户（消息不会被发送）

**DELETE /_synapse/admin/v1/users/{user_id}/shadow_ban**
- 功能：移除影子封禁

**PUT /_synapse/admin/v1/suspend/{user_id}**
- 功能：暂停或恢复用户账户
- 请求体：{ "suspend": true }
- 响应：{ "user_id": "...", "suspended": true }

### 用户查询
**GET /_synapse/admin/v1/whois/{user_id}**
- 功能：获取用户会话信息（IP、设备、连接详情）

**GET /_synapse/admin/v1/users/{user_id}/joined_rooms**
- 功能：获取用户加入的所有房间列表

**GET /_synapse/admin/v1/users/{user_id}/pushers**
- 功能：获取用户的所有推送规则

**GET /_synapse/admin/v1/users/{user_id}/accountdata**
- 功能：获取用户的所有账户数据（全局和房间级别）

### 速率限制
**GET /_synapse/admin/v1/users/{user_id}/override_ratelimit**
- 功能：获取用户的速率限制覆盖设置

**POST /_synapse/admin/v1/users/{user_id}/override_ratelimit**
- 功能：设置用户的速率限制覆盖
- 请求体：{ "messages_per_second": 20, "burst_count": 100 }

**DELETE /_synapse/admin/v1/users/{user_id}/override_ratelimit**
- 功能：删除速率限制覆盖（恢复默认）

## 用户查找模块

**GET /_synapse/admin/v1/auth_providers/{provider}/users/{external_id}**
- 功能：通过认证提供者和外部 ID 查找用户（SSO/OIDC）
- 响应：{ "user_id": "@localuser:server.com" }

**GET /_synapse/admin/v1/threepid/{medium}/users/{address}**
- 功能：通过 3PID（邮箱/手机号）查找用户
- 响应：{ "user_id": "@localuser:server.com" }

## 设备管理模块

**GET /_synapse/admin/v2/users/{user_id}/devices**
- 功能：获取用户的所有设备列表
- 响应：设备数组、总数

**POST /_synapse/admin/v2/users/{user_id}/devices**
- 功能：为用户创建新设备
- 请求体：{ "device_id": "...", "display_name": "..." }

**GET /_synapse/admin/v2/users/{user_id}/devices/{device_id}**
- 功能：获取特定设备的详细信息

**PUT /_synapse/admin/v2/users/{user_id}/devices/{device_id}**
- 功能：更新设备信息（显示名称）
- 请求体：{ "display_name": "..." }

**DELETE /_synapse/admin/v2/users/{user_id}/devices/{device_id}**
- 功能：删除单个设备

**POST /_synapse/admin/v2/users/{user_id}/delete_devices**
- 功能：批量删除多个设备
- 请求体：{ "devices": ["id1", "id2"] }

## 房间管理模块

**GET /_synapse/admin/v1/rooms**
- 功能：获取房间列表（分页、过滤、排序）
- 查询参数：from, limit, order_by, dir, search_term
- 响应：房间数组、总数、分页令牌

**GET /_synapse/admin/v1/rooms/{room_id}**
- 功能：获取单个房间的详细信息

**GET /_synapse/admin/v1/rooms/{room_id}/hierarchy**
- 功能：获取房间的空间层级结构

**GET /_synapse/admin/v1/rooms/{room_id}/members**
- 功能：获取房间成员列表
- 响应：{ "members": [...], "total": N }

**GET /_synapse/admin/v1/rooms/{room_id}/state**
- 功能：获取房间的完整状态事件

**GET /_synapse/admin/v1/rooms/{room_id}/messages**
- 功能：获取房间消息时间线
- 查询参数：from, limit, dir
- 响应：{ "chunk": [...], "start": "...", "end": "..." }

**GET /_synapse/admin/v1/rooms/{room_id}/block**
- 功能：获取房间的封禁状态
- 响应：{ "block": true, "user_id": null }

**PUT /_synapse/admin/v1/rooms/{room_id}/block**
- 功能：设置房间的封禁状态
- 请求体：{ "block": true, "user_id": null }

**GET /_synapse/admin/v1/rooms/{room_id}/forward_extremities**
- 功能：获取房间的前向极端事件（诊断分叉）
- 响应：{ "count": N, "results": [...] }

**DELETE /_synapse/admin/v2/rooms/{room_id}**
- 功能：删除房间（踢出所有成员，可选封禁）
- 请求体：{ "block": true, "purge": true }
- 响应：{ "kicked_users": [...], "failed_to_kick_users": [], "local_aliases": [...], "new_room_id": null }
- 注意：purge=true 当前返回 501，不支持清除事件

## 联邦管理模块

**GET /_synapse/admin/v1/federation/destinations**
- 功能：获取所有联邦目标服务器列表
- 查询参数：from, limit
- 响应：{ "destinations": [...], "total": N, "next_token": "..." }

**GET /_synapse/admin/v1/federation/destinations/{destination}**
- 功能：获取特定服务器的详细信息

**GET /_synapse/admin/v1/federation/destinations/{destination}/rooms**
- 功能：获取与目标服务器共享的房间列表
- 查询参数：from, limit
- 响应：{ "rooms": [...], "total": N, "next_token": "..." }

**POST /_synapse/admin/v1/federation/destinations/{destination}/reset_connection**
- 功能：重置与目标服务器的连接
- 响应：{}

## 统计模块

**GET /_synapse/admin/v1/server_version**
- 功能：获取服务器版本信息
- 响应：{ "server_version": "0.1.0" }

**GET /_synapse/admin/v1/statistics/users/media**
- 功能：获取用户媒体文件统计
- 查询参数：from, limit
- 响应：{ "users": [...], "total": N, "next_token": "..." }
- 注意：可能返回 501

## 事件管理模块

**GET /_synapse/admin/v1/fetch_event/{event_id}**
- 功能：通过事件 ID 获取单个事件的完整信息
- 响应：{ "event": { ... } }

## 事件举报模块

**GET /_synapse/admin/v1/event_reports**
- 功能：获取所有事件举报列表
- 查询参数：from, limit, dir, user_id, room_id, event_sender_user_id
- 响应：{ "event_reports": [...], "total": N, "next_token": "..." }
- 注意：可能返回 501

**GET /_synapse/admin/v1/event_reports/{report_id}**
- 功能：获取特定举报的详细信息

**DELETE /_synapse/admin/v1/event_reports/{report_id}**
- 功能：删除事件举报

## 媒体管理模块

**GET /_synapse/admin/v1/media/{server_name}/{media_id}**
- 功能：获取媒体文件信息
- 响应：{ "media_info": { ... } }
- 注意：可能返回 501

**DELETE /_synapse/admin/v1/media/{server_name}/{media_id}**
- 功能：删除媒体文件
- 响应：{ "deleted_media": [...], "total": N }

**GET /_synapse/admin/v1/room/{room_id}/media**
- 功能：获取房间中的所有媒体文件列表
- 响应：{ "local": [...], "remote": [...] }

**GET /_synapse/admin/v1/users/{user_id}/media**
- 功能：获取用户上传的所有媒体文件列表
- 查询参数：from, limit
- 响应：{ "media": [...], "total": N, "next_token": "..." }

**POST /_synapse/admin/v1/purge_media_cache**
- 功能：清除过期的远程媒体缓存
- 查询参数：before_ts
- 响应：{ "deleted": N }

## 注册管理模块

**GET /_synapse/admin/v1/username_available**
- 功能：检查用户名是否可用
- 查询参数：username
- 响应：{ "available": true }
- 注意：如果不可用返回 400 M_USER_IN_USE

## 计划任务模块

**GET /_synapse/admin/v1/scheduled_tasks**
- 功能：获取计划任务列表
- 查询参数：action_name, resource_id, job_status, max_timestamp
- 响应：{ "scheduled_tasks": [...] }
- 注意：可能返回 501

## 服务器通知模块

**POST /_synapse/admin/v1/send_server_notice**
- 功能：向用户发送服务器通知
- 请求体：{ "user_id": "...", "content": { ... }, "type": "...", "state_key": null }
- 响应：{ "event_id": "$..." }
- 注意：可能返回 501

---

## 认证说明

所有 Admin API 端点（除了登录）都需要管理员权限：

**请求头**：
```
Authorization: Bearer <access_token>
```

## 错误响应格式

```json
{
  "errcode": "M_UNKNOWN",
  "error": "Error description",
  "retry_after_ms": 5000
}
```

常见错误码：
- M_NOT_FOUND: 资源不存在
- M_UNKNOWN: 未知错误
- M_USER_IN_USE: 用户名已被占用
- M_INVALID_PARAM: 参数无效
- M_MISSING_PARAM: 缺少必需参数
- M_UNAUTHORIZED: 未授权或权限不足
- M_NOT_JSON: 请求体不是有效 JSON
- M_BAD_JSON: JSON 格式错误
