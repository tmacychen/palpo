# User Management API Documentation

## Overview

The User Management API provides endpoints for managing Matrix users through the Palpo admin server.

## Base URL

```
http://localhost:8081/api/v1
```

## Authentication

All endpoints require Bearer token authentication in the `Authorization` header:

```
Authorization: Bearer <admin_token>
```

## Endpoints

### User Operations

#### Create User

**POST** `/api/v1/users`

Create a new Matrix user.

**Request Body:**
```json
{
  "user_id": "@username:localhost",
  "displayname": "Display Name",
  "avatar_url": "mxc://...",
  "is_admin": false,
  "is_guest": false,
  "user_type": null,
  "appservice_id": null
}
```

**Response:**
```json
{
  "success": true,
  "user": { ... },
  "generated_password": null,
  "error": null
}
```

---

#### List Users

**GET** `/api/v1/users`

List users with filtering and pagination.

**Query Parameters:**
- `search` (optional): Search by username or display name
- `is_admin` (optional): Filter by admin status
- `is_deactivated` (optional): Filter by deactivation status
- `limit` (optional, default: 50): Number of results
- `offset` (optional, default: 0): Pagination offset
- `sort_by` (optional): Sort field (username, creation_ts, last_seen_ts)
- `sort_order` (optional): asc or desc

**Response:**
```json
{
  "users": [...],
  "total_count": 100,
  "limit": 50,
  "offset": 0
}
```

---

#### Get User

**GET** `/api/v1/users/{user_id}`

Get detailed information about a specific user.

---

#### Update User

**PUT** `/api/v1/users/{user_id}`

Update user information.

**Request Body:**
```json
{
  "displayname": "New Name",
  "avatar_url": "mxc://...",
  "is_admin": true
}
```

---

#### Deactivate User

**DELETE** `/api/v1/users/{user_id}`

Deactivate a user account.

**Request Body:**
```json
{
  "erase": false
}
```

---

#### Reactivate User

**POST** `/api/v1/users/{user_id}/reactivate`

Reactivate a deactivated user.

---

### Password Operations

#### Reset Password

**POST** `/api/v1/users/{user_id}/password`

Reset a user's password.

**Request Body:**
```json
{
  "new_password": "NewSecurePass123!",
  "logout_devices": true
}
```

---

### Device Operations

#### List User Devices

**GET** `/api/v1/users/{user_id}/devices`

List all devices for a user.

#### Delete Device

**DELETE** `/api/v1/users/{user_id}/devices/{device_id}`

Delete a specific device.

#### Batch Delete Devices

**POST** `/api/v1/users/{user_id}/devices/delete`

Delete multiple devices.

---

### Session Operations

#### Get Whois

**GET** `/api/v1/users/{user_id}/whois`

Get connection information for a user.

#### Delete Sessions

**DELETE** `/api/v1/users/{user_id}/sessions`

Delete all sessions for a user.

---

### Rate Limit Operations

#### Get Rate Limit

**GET** `/api/v1/users/{user_id}/rate-limit`

Get custom rate limit configuration.

#### Set Rate Limit

**POST** `/api/v1/users/{user_id}/rate-limit`

Set custom rate limit.

**Request Body:**
```json
{
  "messages_per_second": 100,
  "burst_count": 200
}
```

#### Delete Rate Limit

**DELETE** `/api/v1/users/{user_id}/rate-limit`

Remove custom rate limit.

---

### Shadow Ban Operations

#### Set Shadow Ban

**PUT** `/api/v1/users/{user_id}/shadow-ban`

Set shadow ban status.

**Request Body:**
```json
{
  "shadow_banned": true
}
```

---

### Third-Party ID Operations

#### Get User Threepids

**GET** `/api/v1/users/{user_id}/threepids`

Get user's third-party identifiers.

#### Find User by Threepid

**GET** `/api/v1/threepid/{medium}/users/{address}`

Find a user by their third-party identifier.

#### Get User External IDs

**GET** `/api/v1/users/{user_id}/external_ids`

Get user's SSO external identifiers.

---

### Room Operations

#### List Joined Rooms

**GET** `/api/v1/users/{user_id}/joined_rooms`

List rooms a user has joined.

---

### Media Operations

#### List User Media

**GET** `/api/v1/users/{user_id}/media`

List media files uploaded by a user.

#### Delete User Media

**DELETE** `/api/v1/users/{user_id}/media`

Delete all media for a user.

---

## Error Responses

```json
{
  "success": false,
  "error": "Error message description"
}
```

### Common Status Codes

- `200 OK` - Success
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Missing or invalid authentication
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## Audit Logging

All user management operations are logged to the audit log.