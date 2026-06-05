# InfiniteTypewriter 发布版工作指导

> 目的：如果当前线程因为上下文压缩、工具错误或被迫开新会话，新线程只读本文件，也能理解最终目标、公开仓库边界、旧资料迁移方式、开发分组和验收要求。

更新时间：2026-06-05

旧私有仓库：`<private-old-repo>`

公开发布仓库：`<repo-root>`

## 1. 当前决策

最终目标是公开发布，不再走“先旧工具 v1、再公开 v2”的两阶段路线。

当前路线：

- 旧私有仓库只作为私有原型、拆书资料和迁移来源。
- `InfiniteTypewriter` 是公开发布目标仓库。
- 新仓库不保留旧私有仓库的 git history。
- 新仓库不迁移 `Textbook/`、`analysis/` 私有语料、旧截图、旧构建产物或本机缓存。
- 技术目标一次定为发布架构：Tauri v2 + React/TypeScript + Rust core + SQLite + provider adapter。
- 发布版运行目录采用用户本机 library，不采用旧仓库根 `analysis/<bookId>/` 作为默认运行口径。
- 旧 `analysis/` 只作为本机迁移输入，由导入向导读取、校验、生成迁移报告后写入新 library。

为什么不做两遍：

- 旧 Electron/Node/`analysis/<id>` 路线会形成过渡代码和过渡 schema，最终发布仍要改。
- 公开仓库必须从第一天就隔离私有资料、发布包、许可证、provider 设置和本地用户库。
- 用户最终要的是可发布产品，不是只服务当前私有仓库的中间工具。

## 2. 公开仓库硬边界

### 2.1 私有资料边界

禁止迁入公开仓库：

- `Textbook/`
- `analysis/`
- 源书原文、逐话摘录、私有拆书成品
- 旧盲测、审计、截图、高保真视觉对照、缓存、日志、release 产物
- `.env`、API key、provider token、账号组织信息

允许迁入公开仓库：

- 公开安全的产品目标、架构决策、方法论摘要和 schema 草案
- 不含私有书名、角色名、原文摘录的模板
- 空示例或人工构造的 toy sample
- 迁移工具说明和迁移测试夹具，但夹具不得来自私有语料

### 2.2 GitHub 公开发布边界

- 不 transfer 旧私有仓库。
- 不 mirror 旧私有仓库。
- 不把旧仓库 `.git`、branch、tag、commit history 带到公开仓库。
- 新 GitHub 账号/仓库使用新提交身份和公开仓库专用 remote。
- 公开仓库必须有 `.gitignore` 和边界测试，防止私有资料被误加入。

### 2.3 开发线程边界

- 当前线程是旧仓库中的总览/迁移协调线程。
- 后续真正开发应在 `<repo-root>` 的新会话中执行。
- 未经用户明确批准，不创建 git worktree。
- 新会话开始前必须先读新仓库 `AGENTS.md` 和 `docs/WORKING_GUIDE.md`。

## 3. 产品目标

InfiniteTypewriter 是本地优先小说创作工作台。

核心能力：

- 导入 `txt`、`md`、已有拆解目录。
- 证据式拆书，不做无证据总结。
- 用拆书资料辅助原创规划。
- 用原创资料构建可审计 Context Pack。
- 显式启动 AI job 进行拆解、推演、生成细纲、生成正文、审核回写。
- AI 输出先进入草稿，用户确认后才写入长期资料。
- 支持写后回写、change set、影响范围标记和回头重修。
- 支持人物行动推演，避免为了大纲目标硬控角色。
- 发布版不携带作者私有语料。

AI 是显式任务执行器，不是后台常驻监听器：

- 本地保存只写文件/SQLite，不调用 provider。
- 只有明确 AI 任务按钮才可能消耗 token。
- job 状态轮询、日志查看、SSE/WebSocket 不消耗 token。

## 4. 发布版技术架构

推荐目标：

```text
InfiniteTypewriter/
  src/                         # React/TypeScript UI
  src-tauri/                   # Tauri shell and Rust commands
  crates/
    core/                      # domain model, schema validation, migrations
    providers/                 # provider adapters
  schemas/                     # JSON Schema / TypeScript contracts
  docs/
    WORKING_GUIDE.md
    methodology/
    architecture/
    migration/
  tests/
  .github/
    workflows/
```

运行时用户库：

```text
library/
  sources/       # 用户导入的 txt/md、切片、hash
  analyses/      # Evidence Card、Scene Card、Style Profile、Reference Mechanism
  works/         # 原创书：世界观、人物、大纲、正文、变更集
  runs/          # AI job、日志、草稿、上下文包
  recipes/       # 拆解/创作流程模板
  indexes/       # SQLite、检索缓存、向量索引可选
```

SQLite 用于索引、job、hash、manifest、状态账本和查询；Markdown/自由文本用于正文、文风、世界观长文和复杂技法说明。

## 5. Provider 设计

Provider 从第一天按发布版抽象：

- API Provider：OpenAI-compatible、Anthropic、Gemini、Ollama。
- CLI Agent Provider：Codex、Claude Code。
- Fake/Test Provider：测试、演示、CI。

默认 fail-closed：

- 默认不允许 AI 直接写长期资料。
- 默认不隐藏 CLI 等待权限批准。
- CLI provider 默认只读/非交互/无会话持久化。
- 高级 agent 模式如允许工具、命令或写入，必须单独开关，不进入默认路径。

每个 job 必须记录：

- provider、provider kind、model、认证模式
- CLI/API 参数、cwd、工具权限
- context pack hash、提示词模板版本、输出 schema
- stdout/stderr 或 API response 摘要
- 错误、usage、开始/结束时间、取消状态

## 6. 拆书与原创方法论

拆书是证据式拆解，不是总结式拆解。

每个 Evidence Card 至少包含：

- 原文位置或来源定位
- 观察到的写法
- 该写法在源作中的作用
- 可迁移机制
- 禁止复制项
- 覆盖范围
- 置信度

方法论接口：

- Reader Contract / Promise Progress Payoff
- Scene Card
- POV / 知识边界
- Setting as Pressure
- 分层修订 workflow

迁移原则：

- 参考作品只提供结构、人物、情绪、风格和细节机制。
- 禁止复制原作事件、专有物件、台词、关系链。
- 没有证据的内容只能标记为假设或未覆盖。

## 7. 旧资料安全迁移

旧资料中的 `analysis/` 和 `Textbook/` 不进入公开仓库。

发布版需要实现旧资料导入向导：

1. 用户选择本机旧资料目录。
2. 工具 dry-run 扫描目录和文件类型。
3. 生成迁移清单：来源路径、目标类型、hash、可迁移范围、不可迁移原因。
4. 对已有拆解目录只抽取结构化机制和证据定位，不把源书原文全文带入公开仓库。
5. 用户确认后写入本机 `library/analyses` 或 `library/works`。
6. 生成 migration report。

迁移结果仍是用户本机数据，不提交到 GitHub。

## 8. 前端工作台

发布版首屏不是营销页，而是可用工作台。

建议工作区：

- 拆书工作台：导入源书 -> 选择配方 -> 预览 Source/Context Pack -> 执行 AI -> 审核草稿 -> 发布拆解。
- 原创规划工作台：选择参考机制 -> 生成人物、世界观、大纲、读者契约、场景卡 -> 手动调整。
- 正文创作工作台：选择卷/章/场景 -> 人物行动推演 -> 生成正文草稿 -> 定稿 -> 回写状态和变更集。

所有 AI 按钮必须区别于本地操作按钮，并提示可能消耗 token。

## 9. Implementation Checklist

### 9.0 Public Repository Guardrails

目标：让公开仓库从第一天不可能误提交私有语料。

涉及文件：

- `README.md`
- `AGENTS.md`
- `.gitignore`
- `.github/workflows/*`
- `docs/WORKING_GUIDE.md`
- `tests/repository-boundary.*`

API/schema：无。

测试：

- 断言仓库不存在 `Textbook/`、`analysis/`、`.env`、`*.db`、release/cache/log 产物。
- 断言 package/release 配置不会打包用户 library。
- `tests/repository-boundary.ps1` 是 9.0 的最小可执行边界测试，必须能在尚未创建 package/Tauri 配置时独立运行。
- `.github/workflows/repository-boundary.yml` 在 push/PR 上运行该边界测试。
- `.gitignore` 可以使用等价 glob 写法规避最终扫描误报，但必须用 `git check-ignore` 或边界测试证明仍然生效。

验收命令：

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File tests/repository-boundary.ps1
git status --short --branch
rg -n "Textbook|analysis/|<private-old-repo>|API_KEY|SECRET|TOKEN" .
```

最终 `rg` 只允许命中 `AGENTS.md` 或 `docs/WORKING_GUIDE.md` 中描述公开仓库边界的说明性文字。

不能碰的边界：不复制旧仓库 history，不复制私有语料。

依赖关系：无，第一组执行。

### 9.1 Architecture Scaffold

目标：建立 Tauri + React/TypeScript + Rust core + SQLite 的发布架构骨架。

涉及文件：

- `package.json`
- `src/`
- `src-tauri/`
- `crates/core/`
- `schemas/`
- `tests/`

API/schema：

- `LibraryRoot`
- `LibraryManifest`
- `SourceRecord`
- `WorkRecord`
- `AnalysisRecord`

测试：

- Tauri/Rust unit tests。
- TypeScript typecheck。
- 空 library 初始化。

验收命令：

```powershell
npm run typecheck
npm test
cargo test
```

不能碰的边界：不引入旧 Electron/Node server 作为最终运行壳。

依赖关系：依赖 9.0。

### 9.2 Methodology And Schema Contracts

目标：把拆书/原创核心对象定义成公开安全 schema。

涉及文件：

- `schemas/`
- `crates/core/src/model/`
- `docs/methodology/`

API/schema：

- `EvidenceCard`
- `ReaderContract`
- `SceneCard`
- `KnowledgeBoundary`
- `SettingPressure`
- `ReferenceMechanism`
- `ContextPack`
- `DraftReviewItem`
- `ChangeSet`

测试：

- schema validation。
- fixture round-trip。
- 禁止缺失 evidence/source/coverage/confidence 的 Evidence Card。

验收命令：

```powershell
npm test
cargo test
```

不能碰的边界：公开 fixture 不使用私有书名、角色名、原文摘录。

依赖关系：依赖 9.1。

### 9.3 Private Corpus Migration

目标：把旧 `analysis/` 作为用户本机导入来源，而不是公开仓库内容。

涉及文件：

- `crates/core/src/migration/`
- `src-tauri/src/commands/migration.rs`
- `src/features/migration/`
- `docs/migration/`

API/schema：

- `MigrationDryRun`
- `MigrationSource`
- `MigrationTarget`
- `MigrationReport`

测试：

- toy fixture dry-run。
- 路径穿越防护。
- hash 稳定。
- 不把源书全文写入公开 fixture。

验收命令：

```powershell
npm test
cargo test
rg -n "Textbook|analysis/" .
```

不能碰的边界：不把旧资料复制进 repo。

依赖关系：依赖 9.2。

### 9.4 Provider Adapter And Job Runner

目标：实现 API/CLI/Fake provider 的统一 job 生命周期。

涉及文件：

- `crates/providers/`
- `src-tauri/src/commands/providers.rs`
- `schemas/provider-job.schema.json`
- `tests/providers/`

API/schema：

- `ProviderConfig`
- `ProviderCapability`
- `ProviderJob`
- `ProviderJobEvent`
- `ProviderUsage`

测试：

- fake provider job lifecycle。
- CLI command construction。
- API request construction with redacted secrets。
- fail-closed defaults。

验收命令：

```powershell
npm test
cargo test
```

不能碰的边界：不真实消耗 token 跑自动测试；不记录密钥明文。

依赖关系：依赖 9.2。

### 9.5 Context Pack Builder

目标：每次 AI 任务前构建可审计、可 hash、可预算的上下文包。

涉及文件：

- `crates/core/src/context_pack/`
- `src/features/context-pack/`
- `schemas/context-pack.schema.json`

API/schema：

- `ContextPack`
- `ContextActivation`
- `ContextBudget`
- `ContextHash`

测试：

- 选择清单可审计。
- hash 稳定。
- change set 参与过期设定过滤。
- 本地轮询不触发 provider。

验收命令：

```powershell
npm test
cargo test
```

不能碰的边界：不把完整源书原文默认塞进生成上下文。

依赖关系：依赖 9.2、9.4。

### 9.6 Source Breakdown Wizard

目标：做完整拆书向导 UI。

涉及文件：

- `src/features/source-breakdown/`
- `src/features/jobs/`
- `docs/source-breakdown-wizard-ui-spec.md`

API/schema：

- `SourceImportDraft`
- `BreakdownRecipe`
- `BreakdownTargetSelection`
- `DraftEvidenceCardReviewItem`

测试：

- 本地导入不触发 AI。
- AI 任务按钮提示 token。
- Evidence Card 草稿可接受/编辑/拒绝。

验收命令：

```powershell
npm test
npm run typecheck
```

不能碰的边界：不自动回写长期资料。

依赖关系：依赖 9.3、9.4、9.5。

### 9.7 Original Planning And Writing Workspace

目标：实现原创规划、人物推演、正文草稿、定稿、写后回写。

涉及文件：

- `src/features/workspace/`
- `src/features/writing/`
- `src/features/revisions/`
- `crates/core/src/workspace/`

API/schema：

- `WorkRecord`
- `CharacterStableProfile`
- `CharacterDynamicState`
- `RelationshipTemperature`
- `SceneDraft`
- `RevisionSuggestion`
- `ChangeSet`

测试：

- 人物行动推演检测硬控风险。
- 草稿确认后才定稿。
- 稳定设定变更必须用户确认。
- 回头重修读取最新 change set。

验收命令：

```powershell
npm test
cargo test
npm run typecheck
```

不能碰的边界：不把 AI 推测直接当事实；不把一次临时突破写成永久成长。

依赖关系：依赖 9.5、9.6。

### 9.8 Packaging And Release

目标：公开发布前收敛安装包、许可证、CI 和资料边界。

涉及文件：

- `.github/workflows/`
- `src-tauri/tauri.conf.*`
- `LICENSE`
- `README.md`
- `docs/release-checklist.md`

API/schema：无。

测试：

- 构建产物不含私有资料。
- 安装后创建空用户 library。
- provider secrets redacted。

验收命令：

```powershell
npm run build
cargo test
rg -n "Textbook|analysis/|<private-old-repo>" .
```

不能碰的边界：不发布含私有语料的 artifact。

依赖关系：依赖全部功能组完成。

## 10. 新会话启动规则

在 `InfiniteTypewriter` 新会话中：

1. 进入 `<repo-root>`。
2. 先读 `AGENTS.md`。
3. 再读 `docs/WORKING_GUIDE.md`。
4. 运行 `git status --short --branch`。
5. 不读取或复制 `<private-old-repo>\analysis` / `Textbook`，除非任务明确是“迁移工具 dry-run”，且只作为本机输入。
6. 从 `9.0 Public Repository Guardrails` 开始，不跳组。
7. 每组完成后回报：改了什么、测试命令结果、剩余风险、需要总览线程验收什么。

## 11. 当前待办

- 把本指导迁移到 `<repo-root>\docs\WORKING_GUIDE.md`。
- 在新仓库创建公开安全的 `README.md`、`AGENTS.md`、`.gitignore`。
- 创建公开安全的方法论摘要和迁移说明。
- 在新会话中从 9.0 开始开发。







