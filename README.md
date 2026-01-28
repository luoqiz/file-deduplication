# 文件去重整理工具 (File Deduplication)

一个基于 Tauri + Vue 3 + Rust 的跨平台文件去重和整理工具。快速识别并整理重复文件，按文件类型自动分类。

## ✨ 核心功能

### 🎯 主要特性

- **文件夹选择** - 轻松选择要处理的主目录
- **文件类型分类** - 预设 37 个常见文件扩展名，支持自定义扩展名
- **智能去重** - 通过文件名匹配识别重复文件（支持同名不同扩展的文件对）
- **自动分类整理** - 按文件扩展名自动创建分类目录并移动文件
- **双面板操作** - 分别配置源目录和备份目录，灵活应对不同场景

### 📂 支持的文件类型

**图片**: `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`, `.svg`

**视频**: `.mp4`, `.avi`, `.mkv`, `.mov`, `.flv`, `.wmv`

**音频**: `.mp3`, `.flac`, `.wav`, `.aac`, `.ogg`

**文档**: `.pdf`, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`

**压缩包**: `.zip`, `.rar`, `.7z`, `.tar`, `.gz`

**代码**: `.js`, `.ts`, `.py`, `.java`, `.cpp`, `.c`, `.go`

**其他**: 支持自定义任意扩展名

## 🛠️ 技术栈

### 前端
- **Vue 3** - 现代化的JavaScript框架
- **TypeScript** - 强类型JavaScript
- **Vite** - 新一代前端构建工具
- **Tauri API** - 与后端通信

### 后端
- **Rust** - 高性能、内存安全的系统编程语言
- **Tokio** - 异步运行时
- **RFD (Rust File Dialog)** - 跨平台文件夹选择对话框

## 📋 使用流程

```
1. 点击「选择文件夹」-> 选择包含多个子目录的主目录
                     ↓
2. 应用自动列举主目录下的所有子文件夹
                     ↓
3. 在源面板和备面板中分别配置：
   - 选择要处理的文件类型（支持多选）
   - 选择源目录（文件来源）
   - 选择备目录（备份/副本来源）
                     ↓
4. 点击「确认」触发处理流程：
   - 搜集源目录和备目录中的指定类型文件
   - 匹配同名文件（去重逻辑）
   - 在 move/ 文件夹下按扩展名分类
   - 返回处理结果（成功/失败列表）
```

## 🎨 用户界面

```
┌─────────────────────────────────────────────────┐
│ [选择文件夹] <主目录路径>        [确认]        │
└─────────────────────────────────────────────────┘
┌──────────────────┬────────────────────────────┐
│   源面板         │     备面板                │
│                  │                           │
│ ☑.jpg ☐.png    │ ☑.jpg ☐.png             │
│ ☐.mp4 ☐.zip    │ ☐.mp4 ☐.zip             │
│ ... (横向展示)  │ ... (横向展示)           │
│                  │                           │
│ 自定义: [.txt] [添加]                        │
│ 已选择: .jpg .png                            │
│                  │                           │
│ 📁 folder1      │ 📁 folder1              │
│ 📁 folder2      │ 📁 folder2              │
│ 📁 folder3      │ 📁 folder3              │
└──────────────────┴────────────────────────────┘
```

## 🚀 快速开始

### 前置要求

- Node.js >= 18
- Rust >= 1.70
- pnpm 8.x

### 安装

```bash
# 克隆仓库
git clone <repository-url>
cd file-deduplication

# 安装依赖
pnpm install
```

### 开发模式

```bash
# 启动开发服务器和应用
pnpm tauri:dev
```

### 生产构建

```bash
# 构建应用
pnpm build

# 打包为可执行文件
pnpm tauri build
```

## 📁 项目结构

```
file-deduplication/
├── src/                          # 前端源代码
│   ├── App.vue                  # 主界面组件（选择器、文件夹列表等）
│   ├── main.ts                  # Vue应用入口
│   ├── assets/                  # 静态资源
│   └── vite-env.d.ts            # Vite类型定义
│
├── src-tauri/                    # Tauri后端代码
│   ├── src/
│   │   ├── lib.rs               # 核心业务逻辑（去重算法、文件操作）
│   │   └── main.rs              # Tauri应用入口
│   ├── Cargo.toml               # Rust依赖配置
│   ├── tauri.conf.json          # Tauri应用配置
│   ├── capabilities/            # ACL能力定义
│   ├── icons/                   # 应用图标
│   └── build.rs                 # 构建脚本
│
├── .github/
│   └── workflows/
│       └── release.yml          # 自动发布工作流
│
├── package.json                 # Node.js依赖配置
├── tsconfig.json                # TypeScript配置
├── vite.config.ts               # Vite配置
└── README.md                    # 本文件
```

## 🔧 核心算法

### 去重逻辑

```rust
// 文件匹配算法：比较文件名（不含扩展名）
fn name_matches(file1: &str, file2: &str) -> bool {
    extract_name_without_ext(file1) == extract_name_without_ext(file2)
}

// 例子：
// photo.jpg + photo.png -> 识别为同一文件的不同版本
// document.pdf + document.docx -> 识别为同一文件的不同版本
```

### 文件处理流程

1. **收集文件** - 递归遍历指定目录，收集匹配扩展名的文件
2. **匹配去重** - 对比源目录和备份目录中的文件名（除去扩展名）
3. **创建分类** - 按文件扩展名在 `move/` 文件夹下创建子目录
4. **复制文件** - 将匹配的文件复制到对应的分类目录

## 🤖 自动发布工作流

项目配置了GitHub Actions自动发布工作流（`.github/workflows/release.yml`）。

### 工作流特性

- **自动触发** - 当代码推送到 `main` 分支时自动执行
- **版本管理** - 自动递增小版本号（x.y.z -> x.y.z+1）
- **版本同步** - 同时更新 `package.json` 和 `src-tauri/Cargo.toml`
- **多平台编译** - 在Windows、Ubuntu、macOS上构建
- **自动发布** - 生成GitHub Release并上传编译产物
- **版本跟踪** - 创建Git Tag和自动提交版本更新

## 📝 配置说明

### Tauri配置

应用配置位于 `src-tauri/tauri.conf.json`：

```json
{
  "app": {
    "windows": [
      {
        "title": "文件去重整理工具",
        "width": 1200,
        "height": 800
      }
    ]
  }
}
```

### 应用信息

- **应用名称** - file-deduplication
- **版本** - 0.1.0+（自动递增）
- **标识符** - top.luoqiz.file.deduplication
- **窗口大小** - 1200x800px

## 🐛 常见问题

**Q: 如何自定义文件扩展名？**
A: 在任意面板的"自定义后缀"输入框中输入扩展名（如 `.txt` 或 `txt`），点击"添加"按钮即可。

**Q: 源目录和备目录有什么区别？**
A: 源目录是文件的主要来源，备目录是辅助来源。工具会从两个目录中收集文件，并通过文件名匹配识别重复文件。

**Q: 文件会被删除吗？**
A: 不会。文件被复制到 `move/` 文件夹，原文件保留在原位置。

**Q: move 文件夹在哪里创建？**
A: 在你选择的主文件夹下，会自动创建 `move/` 目录。

## 📄 许可证

MIT License

## 👥 贡献

欢迎提交Issue和Pull Request！

## 📮 联系方式

如有问题或建议，请通过GitHub Issues与我们联系。

---

**最后更新**: 2026年1月28日
