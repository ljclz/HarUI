# HarUI 发布清单（Release Checklist）

> 每次发版按序执行；任一步失败则终止并修复后重来。
> 关联：Keep a Changelog 格式（CHANGELOG.md）、SemVer、12 关门禁。

## 1. 版本号判定（SemVer）

- [ ] 新增公开 API / 新组件 → **minor**（vX.Y+1.0）
- [ ] 仅 bugfix / 性能 / 文档 → **patch**（vX.Y.Z+1）
- [ ] 破坏性变更（删改 pub 项 / view 签名）→ **major** 并在 CHANGELOG 标注迁移指南

## 2. 质量门禁

- [ ] `bash scripts/gate.sh` 12 关全 PASS（fmt/check/clippy -D warnings/test/doc/
      audit/example/占位/组件模式/主题令牌/render/文档一致性）
- [ ] 确认无其他会话并发占用 `CARGO_HOME`/`CARGO_TARGET_DIR`（防锁等待与误报）
- [ ] `cargo bench -p har-ui-components --bench component_bench` 对照
      `docs/perf_baseline.md`，回归 > 2× 需说明或驳回

## 3. 文档同步

- [ ] `CHANGELOG.md` 新增版本条目（Added/Fixed/Changed/Security 分类）
- [ ] `README.md`：版本徽章、状态行、测试数量、新特性要点
- [ ] `RELEASE_NOTES.md` 追加版本段
- [ ] `python scripts/gen_skill_doc.py` 重跑并核对 `docs/skill/har-ui-skills.md`
      （API 漂移检查）
- [ ] `docs/perf_baseline.md` 追加新基线行（含日期与环境）

## 4. 打标签与推送

- [ ] 确认 tag 指向发版最终提交：`git tag -d vX.Y.Z && git tag -a vX.Y.Z -m "..."`
      （**tag 未推送前才允许移动**；已推送的 tag 不可移动，只能出 patch 版）
- [ ] `git push origin main --tags`
- [ ] GitHub Actions `gate.yml` 对 tag 触发的 CI 全绿
- [ ] （可选）GitHub Release：以 CHANGELOG 版本段为说明文本

## 5. 发布后

- [ ] 业务方（菜场收银台）更新依赖引用到新 tag 并编译验证
- [ ] 若含 audit 豁免：确认 `audit.toml` 清理条件是否触发（如 iced 升级）
