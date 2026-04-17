# Tasks - Rust Blog 后端重构

## 任务列表

- [x] Task 1: 初始化Rust项目结构
  - [x] SubTask 1.1: 创建Cargo.toml并配置依赖（axum, tokio, serde, sqlx, sea-orm, jsonwebtoken等）
  - [x] SubTask 1.2: 创建项目目录结构
  - [x] SubTask 1.3: 创建config.toml配置文件

- [x] Task 2: 配置模块实现
  - [x] SubTask 2.1: 实现配置结构体定义
  - [x] SubTask 2.2: 实现TOML配置文件读取

- [x] Task 3: 数据库模块实现
  - [x] SubTask 3.1: 实现SQLite连接管理（使用sqlx + sea-orm）
  - [x] SubTask 3.2: 实现表结构定义（Post, Admin, Message, Tag, Comment, View, Like, Share），使用sea-orm的ActiveModelBehavior自动管理create_at/update_at
  - [x] SubTask 3.3: 实现数据库初始化和迁移

- [ ] Task 4: 数据模型定义
  - [x] SubTask 4.1: 实现Post模型及CRUD，使用sea-orm的ActiveModel自动时间戳
  - [x] SubTask 4.2: 实现Admin模型，使用sea-orm的ActiveModel自动时间戳
  - [x] SubTask 4.3: 实现Message模型，使用sea-orm的ActiveModel自动时间戳
  - [x] SubTask 4.4: 实现Tag/Category/Comment/View/Like/Share模型，使用sea-orm的ActiveModel自动时间戳

- [x] Task 5: 错误处理模块
  - [x] SubTask 5.1: 定义统一错误类型
  - [x] SubTask 5.2: 实现错误转换为API响应

- [x] Task 6: JWT工具模块
  - [x] SubTask 6.1: 实现JWT生成函数
  - [x] SubTask 6.2: 实现JWT验证函数

- [x] Task 7: 中间件实现
  - [x] SubTask 7.1: 实现CORS中间件
  - [x] SubTask 7.2: 实现JWT认证中间件

- [x] Task 8: 响应结构体定义
  - [x] SubTask 8.1: 定义API统一响应结构
  - [x] SubTask 8.2: 定义各业务响应结构

- [ ] Task 9: 业务处理层（Services）
  - [ ] SubTask 9.1: 实现文章业务逻辑
  - [x] SubTask 9.2: 实现认证业务逻辑

- [ ] Task 10: 路由处理函数（Handlers）
  - [x] SubTask 10.1: 实现admin登录处理器
  - [x] SubTask 10.2: 实现article相关处理器
  - [x] SubTask 10.3: 实现message相关处理器
  - [x] SubTask 10.4: 实现dashboard受保护处理器

- [x] Task 11: 路由组装
  - [x] SubTask 11.1: 组装所有路由
  - [x] SubTask 11.2: 应用中间件

- [x] Task 12: 主入口文件
  - [x] SubTask 12.1: 初始化配置
  - [x] SubTask 12.2: 初始化数据库
  - [x] SubTask 12.3: 启动Axum服务

- [x] Task 13: 编译验证
  - [x] SubTask 13.1: 运行cargo check检查编译
  - [x] SubTask 13.2: 运行cargo build构建项目

## 任务依赖
- Task 2 依赖 Task 1
- Task 3 依赖 Task 1, Task 2
- Task 4 依赖 Task 3
- Task 5 独立，可并行
- Task 6 独立，可并行
- Task 7 依赖 Task 5, Task 6
- Task 8 独立，可并行
- Task 9 依赖 Task 4, Task 5
- Task 10 依赖 Task 9, Task 7, Task 8
- Task 11 依赖 Task 10
- Task 12 依赖 Task 11
- Task 13 依赖 Task 12
