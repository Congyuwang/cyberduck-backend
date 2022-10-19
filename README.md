# Backend of Parklife Cyberduck

## Api

### Public Api

Public apis use cookie session id and WeChat Oauth for authentication.  

- login: GET `/login?redirect=https://redirect-after-login.com`
- login callback: GET `login/callback?code=CODE&state=STATE`
- player progress: GET `/api/user-info`
- restart game: DELETE `/api/user-info`
- preview ducks: GET `/api/preview-ducks`
- find duck: GET `/find-duck/:duck_id`

### Admin Api

Admin apis use Bearer Token for authentication.

Admin apis enables CRUD of ducks, locations, exhibits, users, and their relationships.

## Configuration File
file name: config.yaml

### Server Config Template
```yaml
# server_configuration
server_binding: 0.0.0.0:443
admin_token: "a-cryptographically-strong-bearer-token-for-admin-apis"
db_url: "mongodb-url-string"
log_file: "backend.log"
allow_origin: "https://your-front-end-domain.com"

# redis session configuration
redis_session:
  session_secret: "random-64-bytes-in-base64-encoding"
  cookie_name: "some-session-cookie.sid"
  session_expiration: 86400 # 1 day
  redis_url: "redis-url-for-session-storage"

# WeChat config
wechat:
  appid: "wechat-app-id-for-login"
  secret: "wechat-secret-for-login"
  redirect_uri: "https://your-backend/login/callback"

# optional tls securing, omit this part to use http
server_tls:
  cert: "your-tls-cert.crt"
  key: "your-tls-private-kry.key"

```
