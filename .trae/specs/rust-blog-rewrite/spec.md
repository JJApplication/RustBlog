# Rust Blog 后端重构规格说明书

## Why
用户现有Go语言编写的博客后端服务，需要使用Rust语言和Axum框架进行重构，以获得更好的性能和内存安全性。项目采用模块化设计，中间件仅先实现CORS和JWT登录认证，数据库使用SQLite3。

## What Changes
- 使用Rust语言重写整个后端服务
- 使用Axum框架替代Gin框架
- 数据库从MySQL/SQLite混合切换为纯SQLite3
- 配置文件从INI格式迁移到TOML格式
- 实现CORS中间件和JWT标准认证
- 采用现代Rust项目结构，模块独立解耦

## Impact
- 受影响的功能模块：全部后端API
- 受影响的代码：全部Go代码将被Rust替代
- 排除目录：`blogNext`（前端Next.js）、`nextui`（前端NextUI组件库）

## 项目结构
```
Blog/
├── src/
│   ├── main.rs              # 应用入口
│   ├── config.rs           # 配置模块
│   ├── error.rs            # 统一错误处理
│   ├── db/                  # 数据库模块
│   │   ├── mod.rs
│   │   ├── sqlite.rs        # SQLite连接管理
│   │   └── schema.rs        # 表结构定义
│   ├── models/             # 数据模型
│   │   ├── mod.rs
│   │   ├── post.rs         # 文章模型
│   │   ├── tag.rs          # 标签模型
│   │   ├── category.rs     # 分类模型
│   │   ├── comment.rs      # 评论模型
│   │   ├── message.rs      # 留言模型
│   │   ├── view.rs         # 访问量模型
│   │   ├── like.rs         # 点赞模型
│   │   ├── admin.rs        # 管理员模型
│   │   └── response.rs     # API响应结构
│   ├── handlers/           # 路由处理函数
│   │   ├── mod.rs
│   │   ├── article.rs      # 文章相关API
│   │   ├── admin.rs        # 管理员登录API
│   │   ├── message.rs      # 留言API
│   │   ├── tag.rs         # 标签API
│   │   └── dashboard.rs    # 后台管理API
│   ├── middleware/         # 中间件
│   │   ├── mod.rs
│   │   ├── cors.rs         # CORS中间件
│   │   └── auth.rs         # JWT认证中间件
│   ├── services/          # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── article.rs
│   │   └── auth.rs
│   └── utils/             # 工具函数
│       ├── mod.rs
│       └── jwt.rs         # JWT工具
├── Cargo.toml
├── config.toml            # 配置文件
└── blog.db                # SQLite数据库文件
```

## ADDED Requirements

### Requirement: CORS中间件
系统 SHALL 提供跨域资源共享支持，允许前端应用跨域访问API。

#### Scenario: 预检请求处理
- **WHEN** 浏览器发送OPTIONS预检请求
- **THEN** 返回204状态码，设置必要的CORS响应头

#### Scenario: 实际请求处理
- **WHEN** 收到GET/POST/PUT/DELETE请求
- **THEN** 在响应头中包含Access-Control-Allow-Origin等CORS头

### Requirement: JWT认证
系统 SHALL 提供基于JSON Web Token的标准用户认证机制。

#### Scenario: 管理员登录
- **WHEN** 管理员提交正确的用户名和密码
- **THEN** 系统生成JWT token并返回给客户端

#### Scenario: 带Token访问受保护资源
- **WHEN** 请求头中包含有效的Authorization: Bearer <token>
- **THEN** 请求被允许访问受保护的资源

#### Scenario: 无效Token访问受保护资源
- **WHEN** 请求头中Token无效或已过期
- **THEN** 返回401 Unauthorized状态码

### Requirement: 文章管理API
系统 SHALL 提供文章的增删改查API接口。

#### Scenario: 获取文章列表
- **WHEN** GET /api/article/posts
- **THEN** 返回分页后的文章列表

#### Scenario: 获取单个文章
- **WHEN** GET /api/article/post?name=<name>
- **THEN** 返回指定文章的详细信息

### Requirement: 配置文件
系统 SHALL 使用TOML格式配置文件，包含以下配置项：

```toml
[server]
http_port = 5000
read_timeout = 60
write_timeout = 60

[admin]
username = "admin"
password = "12345"
jwt_secret = "your-secret-key"
cookie_max_age = 3600

[database]
path = "blog.db"

[app]
page_size = 8
```

## MODIFIED Requirements

### Requirement: 数据库表结构

所有表都包含以下通用字段：
- `id`: INTEGER，主键自增
- `create_at`: TIMESTAMP，创建时间（由ORM自动设置）
- `update_at`: TIMESTAMP，更新时间（由ORM自动更新）
- `create_by`: INTEGER，创建人ID
- `update_by`: INTEGER，更新人ID

#### Article表 (blog_posts)
| 字段 | 类型 | 说明 |
|------|------|------|
| name | TEXT | 文章唯一标识 |
| title | TEXT | 文章标题 |
| date | TEXT | 发布日期 |
| date_plus | TEXT | 完整日期时间 |
| update_date | TEXT | 更新日期 |
| abstract | TEXT | 文章摘要 |
| content | TEXT | 文章内容 |
| tags | TEXT | 标签列表 |
| categories | TEXT | 分类列表 |
| pin | INTEGER | 是否置顶 |
| lock | INTEGER | 是否锁定 |

#### Admin表 (blog_admin)
| 字段 | 类型 | 说明 |
|------|------|------|
| username | TEXT | 用户名 |
| password | TEXT | 密码 |
| date | TEXT | 创建日期 |

#### Message表 (blog_messages)
| 字段 | 类型 | 说明 |
|------|------|------|
| user | TEXT | 留言用户 |
| date | TEXT | 留言日期 |
| message | TEXT | 留言内容 |

#### Tag表 (blog_tags)
| 字段 | 类型 | 说明 |
|------|------|------|
| tag | TEXT | 标签名 |
| name | TEXT | 关联文章名 |

#### Comment表 (blog_comments)
| 字段 | 类型 | 说明 |
|------|------|------|
| name | TEXT | 关联文章名 |
| user | TEXT | 评论用户 |
| date | TEXT | 评论日期 |
| comment | TEXT | 评论内容 |

#### View表 (blog_views)
| 字段 | 类型 | 说明 |
|------|------|------|
| name | TEXT | 文章名或"all" |
| view | INTEGER | 访问量 |

#### Like表 (blog_likes)
| 字段 | 类型 | 说明 |
|------|------|------|
| name | TEXT | 文章名 |
| like | INTEGER | 点赞数 |

#### Share表 (blog_shares)
| 字段 | 类型 | 说明 |
|------|------|------|
| name | TEXT | 文章名 |
| share | INTEGER | 分享数 |

## API路由设计

### 公开API（无需认证）
- `GET /hello` - 健康检查
- `GET /api/article/posts` - 获取文章列表
- `GET /api/article/post?name=<name>` - 获取单篇文章
- `GET /api/article/tags` - 获取所有标签
- `GET /api/article/tag?name=<tag>` - 按标签获取文章
- `GET /api/article/brother?name=<name>` - 获取文章上下篇
- `GET /api/article/search?keyword=<kw>` - 搜索文章
- `GET /api/article/comments?name=<name>` - 获取评论
- `POST /api/article/comments` - 添加评论
- `GET /api/article/likes?name=<name>` - 获取点赞数
- `POST /api/article/likes` - 添加点赞
- `GET /api/article/views?name=<name>` - 获取访问量
- `GET /api/article/archive` - 获取归档列表
- `GET /api/article/archives?date=<date>` - 获取归档文章
- `GET /api/message` - 获取留言列表
- `POST /api/message` - 添加留言
- `POST /api/admin/login` - 管理员登录

### 受保护API（需要JWT认证）
- `POST /api/dashboard/post` - 创建文章
- `PUT /api/dashboard/post` - 更新文章
- `DELETE /api/dashboard/post` - 删除文章
- `POST /api/dashboard/tag` - 创建标签
- `PUT /api/dashboard/tag` - 更新标签
- `DELETE /api/dashboard/tag` - 删除标签
- `PUT /api/dashboard/comment` - 更新评论
- `DELETE /api/dashboard/comment` - 删除评论
- `PUT /api/dashboard/message` - 更新留言
- `DELETE /api/dashboard/message` - 删除留言

## 错误处理
系统 SHALL 使用统一的错误响应格式：

```json
{
  "code": 200,
  "msg": "success",
  "data": null
}
```

错误码定义：
- 200: 成功
- 400: 请求参数错误
- 401: 未授权/认证失败
- 403: 禁止访问
- 404: 资源不存在
- 500: 服务器内部错误
