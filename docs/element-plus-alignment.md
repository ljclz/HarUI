# Element Plus 对齐评估

> 评估日期：2026-10-10 ｜ 对比基准：Element Plus 2.x（约 70 组件，不含 Border/Color 纯样式文档页）
> 数据来源：本机源码实证（`crates/components/src/lib.rs` 权威组件清单 + 全源码 grep 核对），非文档转抄
> 结论先行：**高频核心组件 ~87% 对齐且 API 语义忠实；10 个真实缺口；图标覆盖 40/~290 为最大硬伤；像素级视觉对齐未验证**
>
> **✅ P1 补齐已实施（2026-10-10）**：新增 Autocomplete / Image / TreeSelect 三组件
> （64 组件），图标扩至 **333 个**（EP 全集 293 + legacy 40，scripts/gen_icons.py
> 从 iconify 官方数据生成）并挂上 iced svg 渲染路径（此前图标系统为纯数据无渲染）。
> 缺口余 7 个低频件（见 1.2 表，排除已补三项）。

## 一、组件覆盖矩阵

### 1.1 完全对齐（57 项，✅）

| EP 分类 | HarUI 对齐组件（模块名） |
|---|---|
| 基础 | button、icon（core 内嵌 40 SVG）、grid（对标 row/col 含响应式断点）、link、scrollbar、space、text、divider、container（core/layout/container.rs，对标 el-container 五件套） |
| 表单 | cascader、checkbox、color_picker、date_picker（含 date/datetime/month/year/daterange 类型变体）、form、input、input_number、rate、select、slider、switch、time_picker、upload |
| 数据展示 | avatar、badge、calendar、card、carousel、collapse、descriptions、empty、pagination、progress、result_page、skeleton、statistic、table（内置虚拟滚动 ≈ Table V2 核心能力 + 冻结列/列宽拖拽）、tag、timeline、tree |
| 导航 | affix、backtop、breadcrumb、dropdown、menu、page_header、steps、tabs |
| 反馈 | alert、dialog、drawer、message、message_box、notification、popconfirm、popover、tooltip（12 方位 placement 对齐） |
| 自有扩展 | keypad、payment、hang_order、customer_display、status_bar（EP 无的 POS 场景组件） |

### 1.2 缺口清单（原 10 项，grep 全源码实证）

> **状态（2026-10-10，W9-P1/P2 后）**：已补 7 项（Autocomplete/Transfer/TreeSelect/Image/Watermark/Anchor/Countdown ✅），余 3 项低频挂起（Tour/InputTag/Splitter）。

| 缺失组件 | 严重度 | 替代方案 / 说明 |
|---|---|---|
| Autocomplete | **中** | ✅ 已补（W9-P1）：本地过滤/自定义 provider/键盘选择 |
| Transfer（穿梭框） | 中 | ✅ 已补（W9-P2）：双面板勾选/搬移/禁用守卫 |
| TreeSelect | 中 | ✅ 已补（W9-P1）：勾选镜像单选/面板开合/树消息透传 |
| Image（预览/懒加载） | **中** | ✅ 已补（W9-P1）：加载状态机/失败重试/预览开关 |
| Watermark | 低 | ✅ 已补（W9-P2）：居中半透明层（平铺+旋转待 canvas） |
| Anchor | 低 | ✅ 已补（W9-P2）：点击高亮/滚动联动 |
| Tour | 低 | — |
| Countdown | 低 | ✅ 已补（W9-P2）：Tick 驱动/暂停恢复/0 时长语义 |
| InputTag | 低 | EP 2.9+ 新增 |
| Splitter | 低 | EP 2.8+ 新增 |

### 1.3 部分等价（形态不同，⚠️）

| EP 组件 | HarUI 等价物 | 差距说明 |
|---|---|---|
| Table V2 | table（内置虚拟滚动 + 冻结列 + 拖拽） | 核心能力等价，行列虚拟化细节未完全对标 |
| Tree V2 | tree | 大数据量树虚拟化未做 |
| Select V2 | select | 大选项量虚拟化未做 |
| DateTime Picker / Time Select | date_picker 类型变体 | DateTime 已支持；Time Select（固定时刻下拉）无 |
| Infinite Scroll | table 虚拟滚动 / scrollable | 指令形态无，能力部分覆盖 |
| ConfigProvider | Theme 全局体系 | 概念等价，形态不同（无逐层配置） |

## 二、对齐质量抽查（实证结果）

- **API 语义**：12 方位 placement、ButtonType 六型、Table 列 fixed/sortable/resizable、Form 的 required/rules/validator 三层校验——与 EP 命名和语义一一对应；源码 **55 个文件、51 处**注释直接标注对照的 `el-xxx` 标签名
- **设计令牌**：EP 色板（primary/success/warning/danger/info + 9 级明暗 + 中性色 8 级）、间距/圆角/阴影/z-index/排版全套复刻；硬编码颜色 0（全部走令牌）
- **交互状态**：hover/active/disabled/loading 守卫、IME 中文输入状态机——EP 交互语义已映射
- **超出 EP**：POS 五件套、Rust 内存安全、wasm 双平台（61 组件 wasm32 编译零错误并公网画廊验证）

## 三、两个未对齐的硬项

### 3.1 图标覆盖：40 / ~290（最大硬伤）

- EP 配套 `@element-plus/icons-vue` 约 **290 个**图标；HarUI `IconName` 枚举实测 **40 个**
- 建议补齐方式：从 iconify 的 EP 图标集批量生成 Rust 枚举 + 内嵌 SVG（脚本化，1 会话量级），命名对齐 EP 图标名

### 3.2 像素级视觉对齐：未验证

- 全部 T3 截图测试为**自建基准**（与自身历史帧比对），从未与 EP 官方网页渲染做 AE<100 / SSIM>0.95 对照
- "85-95% 还原度"目前是设计文档的自我声明等级
- 补齐方式：搭 EP 官网组件对照页截图管线（1-2 会话），含 100%/125% 缩放两档

## 四、补齐路线建议（按需启动，非当前迭代）

| 优先级 | 项 | 预算 |
|---|---|---|
| P1 | Autocomplete + Image + TreeSelect 三个中频缺口 | 1-2 会话 |
| P1 | 图标批量扩充至 EP 全集（iconify 脚本生成） | 1 会话 |
| P2 | Transfer / Watermark / Anchor / Countdown 等低频件 | 1 会话 |
| P2 | EP 像素级视觉对照管线 | 1-2 会话 |
| 挂起 | Select V2 / Tree V2 / Splitter / InputTag（等业务触发） | — |

> 触发条件：菜场收银台业务如需后台管理端（库存/报表页），P1 项建议随业务先行补齐；
> 纯收银场景（POS 五件套 + 现有 61 组件）已完全够用。
