//! T5 Stress 性能基准 — criterion bench
//!
//! 对代表性组件的核心操作做微基准，验证：
//! 1. handle 单次调用延迟在微秒级
//! 2. 大批量事件序列吞吐稳定
//! 3. 大规模数据（1000+ items / 1000+ 节点）下无性能塌方
//!
//! 覆盖组件：Slider、Cascader、Collapse、Steps、Upload、Rate、Progress

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use har_ui_components::cascader::{Cascader, CascaderMessage, CascaderNode};
use har_ui_components::collapse::{Collapse, CollapseItem, CollapseMessage};
use har_ui_components::progress::{Progress, ProgressMessage};
use har_ui_components::rate::{Rate, RateMessage};
use har_ui_components::slider::{Slider, SliderMessage};
use har_ui_components::steps::{Step, Steps, StepsMessage};
use har_ui_components::upload::{Upload, UploadFile, UploadMessage};

// ===================== Slider =====================

fn bench_slider_handle_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("slider/handle_single");
    for &step in &[1.0_f64, 0.1, 0.01] {
        group.bench_with_input(BenchmarkId::from_parameter(step), &step, |b, &step| {
            b.iter(|| {
                let mut s = Slider::new()
                    .with_min(0.0)
                    .with_max(100.0)
                    .with_step(step);
                s.handle(SliderMessage::SetValue(42.0));
                s.handle(SliderMessage::Increase);
                s.handle(SliderMessage::Decrease);
                s.handle(SliderMessage::SetRange(10.0, 90.0));
                black_box(s.value());
            });
        });
    }
    group.finish();
}

fn bench_slider_handle_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("slider/handle_batch");
    for &n in &[100usize, 1000, 10000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut s = Slider::new().with_min(0.0).with_max(1000.0).with_step(1.0);
                for i in 0..n {
                    s.handle(SliderMessage::SetValue((i % 1000) as f64));
                }
                black_box(s.value());
            });
        });
    }
    group.finish();
}

// ===================== Rate =====================

fn bench_rate_handle_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate/handle_batch");
    for &n in &[100usize, 1000, 10000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut r = Rate::new().with_max(10).with_allow_half(true);
                for i in 0..n {
                    let v = ((i % 20) as f64) * 0.5;
                    r.handle(RateMessage::SetValue(v));
                }
                black_box(r.value());
            });
        });
    }
    group.finish();
}

// ===================== Progress =====================

fn bench_progress_handle_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("progress/handle_batch");
    for &n in &[100usize, 1000, 10000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut p = Progress::new();
                for i in 0..n {
                    p.handle(ProgressMessage::Increment((i % 5) as i32));
                }
                black_box(p.percentage());
            });
        });
    }
    group.finish();
}

// ===================== Cascader =====================

/// 构造深度为 depth、每层 fanout 个子节点的树
fn build_tree(depth: usize, fanout: usize) -> Vec<CascaderNode> {
    fn build(prefix: &str, depth: usize, fanout: usize, counter: &mut usize) -> Vec<CascaderNode> {
        (0..fanout)
            .map(|_| {
                let value = format!("{}_{}_{}", prefix, depth, counter.clone());
                *counter += 1;
                let label = format!("L_{}", value);
                let children = if depth > 0 {
                    build(&value, depth - 1, fanout, counter)
                } else {
                    Vec::new()
                };
                CascaderNode::new(value, label).with_children(children)
            })
            .collect()
    }
    let mut counter = 0;
    build("root", depth, fanout, &mut counter)
}

/// 收集所有叶子路径（用于 Select 消息）
fn collect_leaf_paths(nodes: &[CascaderNode]) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    fn walk(nodes: &[CascaderNode], prefix: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
        for n in nodes {
            prefix.push(n.value().to_string());
            if n.children().is_empty() {
                paths.push(prefix.clone());
            } else {
                walk(n.children(), prefix, paths);
            }
            prefix.pop();
        }
    }
    walk(nodes, &mut Vec::new(), &mut paths);
    paths
}

fn bench_cascader_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("cascader/select");
    for &(depth, fanout) in &[(2usize, 3usize), (3, 4), (4, 5)] {
        let tree = build_tree(depth, fanout);
        let paths = collect_leaf_paths(&tree);
        let label = format!("d{}_f{}", depth, fanout);
        group.bench_with_input(BenchmarkId::from_parameter(label), &paths, |b, paths| {
            b.iter(|| {
                let mut casc = Cascader::new().with_options(tree.clone());
                for path in paths.iter() {
                    if let Some(last) = path.last() {
                        casc.handle(CascaderMessage::Select(last.clone()));
                    }
                }
                black_box(casc.selected_path());
            });
        });
    }
    group.finish();
}

fn bench_cascader_select_batch(c: &mut Criterion) {
    let tree = build_tree(3, 4);
    let paths = collect_leaf_paths(&tree);
    let mut group = c.benchmark_group("cascader/select_batch");
    for &n in &[100usize, 1000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut casc = Cascader::new().with_options(tree.clone());
                for i in 0..n {
                    let path = &paths[i % paths.len()];
                    if let Some(last) = path.last() {
                        casc.handle(CascaderMessage::Select(last.clone()));
                    }
                }
                black_box(casc.selected_path());
            });
        });
    }
    group.finish();
}

// ===================== Collapse =====================

fn bench_collapse_toggle_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("collapse/toggle_batch");
    for &n_items in &[10usize, 100, 1000] {
        group.throughput(Throughput::Elements((n_items * 10) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_items),
            &n_items,
            |b, &n_items| {
                b.iter(|| {
                    let mut col = Collapse::new();
                    for i in 0..n_items {
                        col.add_item(CollapseItem::new(
                            format!("k_{}", i),
                            format!("标题{}", i),
                        ));
                    }
                    // 每项 Toggle 10 次
                    for i in 0..n_items * 10 {
                        let key = format!("k_{}", i % n_items);
                        col.handle(CollapseMessage::Toggle(key));
                    }
                    black_box(col.active_keys());
                });
            },
        );
    }
    group.finish();
}

// ===================== Steps =====================

fn bench_steps_nav_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("steps/nav_batch");
    for &n_steps in &[10usize, 100, 1000] {
        group.throughput(Throughput::Elements((n_steps * 5) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(n_steps),
            &n_steps,
            |b, &n_steps| {
                b.iter(|| {
                    let mut s = Steps::new();
                    for i in 0..n_steps {
                        s = s.with_step(Step::new(format!("步骤{}", i + 1)));
                    }
                    // 顺序 Next 走完，再 Prev 回退
                    for _ in 0..n_steps {
                        s.handle(StepsMessage::Next);
                    }
                    for _ in 0..n_steps {
                        s.handle(StepsMessage::Prev);
                    }
                    // 跳跃
                    for i in 0..n_steps {
                        s.handle(StepsMessage::JumpTo(i % n_steps));
                    }
                    black_box(s.current());
                });
            },
        );
    }
    group.finish();
}

// ===================== Upload =====================

fn bench_upload_add_remove_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("upload/add_remove_batch");
    for &n in &[100usize, 1000, 5000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                let mut u = Upload::new().with_multiple(true);
                for i in 0..n {
                    u.handle(UploadMessage::AddFile(UploadFile::new(
                        format!("f_{}.jpg", i),
                        1024,
                    )));
                }
                // 删除一半
                for _ in 0..n / 2 {
                    u.handle(UploadMessage::Remove(0));
                }
                black_box(u.file_list().len());
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_slider_handle_single,
    bench_slider_handle_batch,
    bench_rate_handle_batch,
    bench_progress_handle_batch,
    bench_cascader_select,
    bench_cascader_select_batch,
    bench_collapse_toggle_batch,
    bench_steps_nav_batch,
    bench_upload_add_remove_batch,
);
criterion_main!(benches);
