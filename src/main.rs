#![cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]

use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Benchmark {
    device: String,
    benchmark: String,
    score: u32,
    cost_per_run_cents: u32,
    test_date: String,
}

fn format_cost(cents: u32) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}

fn sample_data() -> Vec<Benchmark> {
    vec![
        (
            "AWS c7g.large (Graviton3)",
            "Geekbench 6 Multi",
            16320,
            68,
            "2026-02-03",
        ),
        (
            "AWS c7g.xlarge (Graviton3)",
            "Geekbench 6 Multi",
            31840,
            91,
            "2026-02-03",
        ),
        (
            "AWS c8g.large (Graviton4)",
            "Geekbench 6 Multi",
            18150,
            64,
            "2026-02-05",
        ),
        (
            "AWS c8g.2xlarge (Graviton4)",
            "Geekbench 6 Multi",
            70210,
            138,
            "2026-02-05",
        ),
        (
            "AWS c7i.large (Xeon Sapphire Rapids)",
            "Geekbench 6 Multi",
            15480,
            79,
            "2026-02-07",
        ),
        (
            "AWS c7i.2xlarge (Xeon Sapphire Rapids)",
            "Geekbench 6 Multi",
            60120,
            156,
            "2026-02-07",
        ),
        (
            "AWS m7a.xlarge (EPYC Genoa)",
            "Geekbench 6 Multi",
            28440,
            118,
            "2026-02-08",
        ),
        (
            "AWS r7a.2xlarge (EPYC Genoa)",
            "SPECint2017 Rate",
            492,
            142,
            "2026-02-10",
        ),
        (
            "AWS c7gn.4xlarge (Graviton3E)",
            "OpenSSL RSA Sign",
            42800,
            129,
            "2026-02-11",
        ),
        (
            "AWS c7gn.8xlarge (Graviton3E)",
            "NGINX Req/s",
            186000,
            214,
            "2026-02-11",
        ),
        (
            "AWS m8g.xlarge (Graviton4)",
            "SPECint2017 Rate",
            402,
            86,
            "2026-02-14",
        ),
        (
            "AWS m8g.4xlarge (Graviton4)",
            "PostgreSQL TPS",
            22850,
            151,
            "2026-02-14",
        ),
        (
            "AWS r8g.2xlarge (Graviton4)",
            "7-Zip Compression",
            133400,
            144,
            "2026-02-16",
        ),
        (
            "AWS c7i.4xlarge (Xeon Sapphire Rapids)",
            "SPECint2017 Rate",
            918,
            208,
            "2026-02-18",
        ),
        (
            "AWS c7a.4xlarge (EPYC Genoa)",
            "SPECint2017 Rate",
            944,
            199,
            "2026-02-18",
        ),
        (
            "AWS c7a.8xlarge (EPYC Genoa)",
            "NGINX Req/s",
            255000,
            286,
            "2026-02-20",
        ),
        (
            "AWS r7iz.2xlarge (Xeon Sapphire Rapids)",
            "PostgreSQL TPS",
            19420,
            167,
            "2026-02-20",
        ),
        (
            "AWS i4i.2xlarge (Xeon Ice Lake)",
            "OpenSSL RSA Sign",
            29600,
            174,
            "2026-02-21",
        ),
        (
            "AWS c6a.4xlarge (EPYC Milan)",
            "7-Zip Compression",
            121900,
            187,
            "2026-02-22",
        ),
        (
            "AWS m7i.2xlarge (Xeon Sapphire Rapids)",
            "PostgreSQL TPS",
            17340,
            149,
            "2026-02-23",
        ),
        (
            "AWS r7i.4xlarge (Xeon Sapphire Rapids)",
            "SPECint2017 Rate",
            901,
            213,
            "2026-02-24",
        ),
        (
            "AWS m7g.2xlarge (Graviton3)",
            "OpenSSL RSA Sign",
            31100,
            104,
            "2026-02-25",
        ),
        (
            "AWS c6g.4xlarge (Graviton2)",
            "NGINX Req/s",
            142800,
            132,
            "2026-02-26",
        ),
        (
            "AWS c7g.16xlarge (Graviton3)",
            "7-Zip Compression",
            418600,
            322,
            "2026-02-28",
        ),
        (
            "AWS c7i.16xlarge (Xeon Sapphire Rapids)",
            "Geekbench 6 Multi",
            462300,
            398,
            "2026-02-28",
        ),
        (
            "GCP t2a-standard-4 (Ampere Altra)",
            "Geekbench 6 Multi",
            14820,
            56,
            "2026-03-02",
        ),
        (
            "GCP t2a-standard-8 (Ampere Altra)",
            "Geekbench 6 Multi",
            28740,
            89,
            "2026-03-02",
        ),
        (
            "GCP c3-standard-4 (Xeon Sapphire Rapids)",
            "Geekbench 6 Multi",
            16980,
            71,
            "2026-03-04",
        ),
        (
            "GCP c3-standard-8 (Xeon Sapphire Rapids)",
            "Geekbench 6 Multi",
            33120,
            109,
            "2026-03-04",
        ),
        (
            "GCP c3d-standard-8 (EPYC Genoa)",
            "SPECint2017 Rate",
            534,
            116,
            "2026-03-06",
        ),
        (
            "GCP c3d-standard-16 (EPYC Genoa)",
            "SPECint2017 Rate",
            1038,
            201,
            "2026-03-06",
        ),
        (
            "GCP c4-standard-8 (Xeon Emerald Rapids)",
            "Geekbench 6 Multi",
            35640,
            112,
            "2026-03-08",
        ),
        (
            "GCP c4-standard-16 (Xeon Emerald Rapids)",
            "Geekbench 6 Multi",
            69480,
            191,
            "2026-03-08",
        ),
        (
            "GCP c4a-standard-8 (EPYC Turin)",
            "Geekbench 6 Multi",
            37210,
            104,
            "2026-03-10",
        ),
        (
            "GCP c4a-standard-16 (EPYC Turin)",
            "PostgreSQL TPS",
            24760,
            188,
            "2026-03-10",
        ),
        (
            "GCP n4-standard-8 (Xeon Emerald Rapids)",
            "PostgreSQL TPS",
            18810,
            128,
            "2026-03-12",
        ),
        (
            "GCP n4-standard-16 (Xeon Emerald Rapids)",
            "OpenSSL RSA Sign",
            38700,
            187,
            "2026-03-12",
        ),
        (
            "GCP c2d-standard-16 (EPYC Milan)",
            "7-Zip Compression",
            149500,
            229,
            "2026-03-14",
        ),
        (
            "GCP h3-standard-8 (Xeon Sapphire Rapids)",
            "NGINX Req/s",
            174300,
            162,
            "2026-03-15",
        ),
        (
            "GCP h3-standard-16 (Xeon Sapphire Rapids)",
            "OpenSSL RSA Sign",
            51200,
            247,
            "2026-03-15",
        ),
        (
            "GCP c3-highcpu-22 (Xeon Sapphire Rapids)",
            "SPECint2017 Rate",
            1310,
            264,
            "2026-03-17",
        ),
        (
            "GCP c3-highcpu-44 (Xeon Sapphire Rapids)",
            "NGINX Req/s",
            291400,
            381,
            "2026-03-17",
        ),
        (
            "GCP t2a-standard-16 (Ampere Altra)",
            "7-Zip Compression",
            221900,
            136,
            "2026-03-19",
        ),
        (
            "GCP c4-standard-32 (Xeon Emerald Rapids)",
            "OpenSSL RSA Sign",
            72800,
            306,
            "2026-03-20",
        ),
        (
            "GCP c4a-highcpu-32 (EPYC Turin)",
            "SPECint2017 Rate",
            2042,
            278,
            "2026-03-20",
        ),
        (
            "GCP c3d-highcpu-30 (EPYC Genoa)",
            "NGINX Req/s",
            318700,
            289,
            "2026-03-22",
        ),
        (
            "GCP n4-highmem-32 (Xeon Emerald Rapids)",
            "PostgreSQL TPS",
            33280,
            301,
            "2026-03-23",
        ),
        (
            "GCP c2d-highcpu-32 (EPYC Milan)",
            "Geekbench 6 Multi",
            108400,
            334,
            "2026-03-24",
        ),
        (
            "GCP c4a-standard-48 (EPYC Turin)",
            "7-Zip Compression",
            438900,
            366,
            "2026-03-25",
        ),
        (
            "GCP c4-standard-48 (Xeon Emerald Rapids)",
            "SPECint2017 Rate",
            2860,
            402,
            "2026-03-25",
        ),
    ]
    .into_iter()
    .map(
        |(device, benchmark, score, cost_per_run_cents, test_date)| Benchmark {
            device: device.into(),
            benchmark: benchmark.into(),
            score,
            cost_per_run_cents,
            test_date: test_date.into(),
        },
    )
    .collect()
}

#[derive(Clone, Debug, PartialEq)]
struct BenchmarkDetails {
    provider: String,
    instance_type: String,
    cpu_model: String,
    cpu_arch: String,
    vcpus: u32,
    memory_gib: u32,
    region: String,
    operating_system: String,
    runtime_secs: u32,
    hourly_cost_cents: u32,
    samples: u32,
    benchmark_focus: String,
    notes: String,
}

fn format_duration(seconds: u32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;

    if minutes == 0 {
        format!("{}s", seconds)
    } else {
        format!("{}m {}s", minutes, remaining_seconds)
    }
}

fn extract_provider(device: &str) -> String {
    device.split_whitespace().next().unwrap_or("Unknown").into()
}

fn extract_instance_type(device: &str) -> String {
    device
        .split_once(' ')
        .map(|(_, rest)| rest.split(" (").next().unwrap_or(rest).to_string())
        .unwrap_or_else(|| device.to_string())
}

fn extract_cpu_model(device: &str) -> String {
    device
        .split_once('(')
        .and_then(|(_, rest)| rest.strip_suffix(')'))
        .unwrap_or("Unknown CPU")
        .to_string()
}

fn detect_cpu_arch(cpu_model: &str) -> String {
    if cpu_model.contains("Graviton") || cpu_model.contains("Ampere") {
        "Arm64".into()
    } else {
        "x86_64".into()
    }
}

fn estimate_vcpus(provider: &str, instance_type: &str) -> u32 {
    if provider == "AWS" {
        let size = instance_type.rsplit('.').next().unwrap_or_default();
        match size {
            "large" => 2,
            "xlarge" => 4,
            _ if size.ends_with("xlarge") => size
                .trim_end_matches("xlarge")
                .parse::<u32>()
                .map(|multiplier| multiplier * 4)
                .unwrap_or(4),
            _ => 4,
        }
    } else {
        instance_type
            .rsplit('-')
            .next()
            .and_then(|count| count.parse::<u32>().ok())
            .unwrap_or(4)
    }
}

fn estimate_memory_gib(instance_type: &str, vcpus: u32) -> u32 {
    let gib_per_vcpu = if instance_type.contains("highmem") || instance_type.starts_with('r') {
        8
    } else if instance_type.contains("highcpu") {
        1
    } else if instance_type.starts_with('m') || instance_type.starts_with('n') {
        4
    } else if instance_type.starts_with('i') {
        8
    } else {
        2
    };

    vcpus * gib_per_vcpu
}

fn benchmark_runtime_secs(benchmark_name: &str) -> u32 {
    match benchmark_name {
        "Geekbench 6 Multi" => 300,
        "SPECint2017 Rate" => 900,
        "OpenSSL RSA Sign" => 240,
        "NGINX Req/s" => 420,
        "PostgreSQL TPS" => 600,
        "7-Zip Compression" => 480,
        _ => 360,
    }
}

fn estimated_hourly_cost_cents(cost_per_run_cents: u32, runtime_secs: u32) -> u32 {
    (cost_per_run_cents * 3600).div_ceil(runtime_secs)
}

fn pick_region(provider: &str, score: u32, test_date: &str) -> String {
    let aws_regions = ["us-east-1", "eu-west-1", "ap-southeast-2"];
    let gcp_regions = ["us-central1", "europe-west4", "asia-southeast1"];
    let day = test_date
        .rsplit('-')
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let index = (score as usize + day) % 3;

    if provider == "AWS" {
        aws_regions[index].into()
    } else {
        gcp_regions[index].into()
    }
}

fn benchmark_focus(benchmark_name: &str) -> String {
    match benchmark_name {
        "Geekbench 6 Multi" => "Broad application throughput".into(),
        "SPECint2017 Rate" => "Integer-heavy compute throughput".into(),
        "OpenSSL RSA Sign" => "Cryptographic signing throughput".into(),
        "NGINX Req/s" => "HTTP serving capacity under concurrent load".into(),
        "PostgreSQL TPS" => "Transactional database throughput".into(),
        "7-Zip Compression" => "Compression speed and memory efficiency".into(),
        _ => "Synthetic benchmark profile".into(),
    }
}

fn build_benchmark_details(benchmark: &Benchmark) -> BenchmarkDetails {
    let provider = extract_provider(&benchmark.device);
    let instance_type = extract_instance_type(&benchmark.device);
    let cpu_model = extract_cpu_model(&benchmark.device);
    let cpu_arch = detect_cpu_arch(&cpu_model);
    let vcpus = estimate_vcpus(&provider, &instance_type);
    let memory_gib = estimate_memory_gib(&instance_type, vcpus);
    let runtime_secs = benchmark_runtime_secs(&benchmark.benchmark);
    let hourly_cost_cents = estimated_hourly_cost_cents(benchmark.cost_per_run_cents, runtime_secs);
    let region = pick_region(&provider, benchmark.score, &benchmark.test_date);
    let operating_system = if provider == "AWS" {
        "Amazon Linux 2023".to_string()
    } else {
        "Debian 12".to_string()
    };
    let samples = 5 + (benchmark.score % 3);
    let benchmark_focus = benchmark_focus(&benchmark.benchmark);
    let notes = format!(
        "Generated synthetic drill-down data for {} on {}. This detail view is intended for UI exploration only.",
        benchmark.benchmark, benchmark.device
    );

    BenchmarkDetails {
        provider,
        instance_type,
        cpu_model,
        cpu_arch,
        vcpus,
        memory_gib,
        region,
        operating_system,
        runtime_secs,
        hourly_cost_cents,
        samples,
        benchmark_focus,
        notes,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SortField {
    Device,
    Benchmark,
    Score,
    CostPerRun,
    TestDate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SortConfig {
    field: SortField,
    ascending: bool,
}

fn toggle_sort(current: Option<SortConfig>, field: SortField) -> Option<SortConfig> {
    match current {
        Some(config) if config.field == field => Some(SortConfig {
            field,
            ascending: !config.ascending,
        }),
        _ => Some(SortConfig {
            field,
            ascending: true,
        }),
    }
}

fn sort_indicator(current: Option<SortConfig>, field: SortField) -> &'static str {
    match current {
        Some(config) if config.field == field && config.ascending => " ↑",
        Some(config) if config.field == field => " ↓",
        None | Some(_) => "",
    }
}

fn sort_benchmarks(items: &mut [Benchmark], config: SortConfig) {
    items.sort_by(|left, right| {
        let ordering = match config.field {
            SortField::Device => left.device.to_lowercase().cmp(&right.device.to_lowercase()),
            SortField::Benchmark => left
                .benchmark
                .to_lowercase()
                .cmp(&right.benchmark.to_lowercase()),
            SortField::Score => left.score.cmp(&right.score),
            SortField::CostPerRun => left.cost_per_run_cents.cmp(&right.cost_per_run_cents),
            SortField::TestDate => left.test_date.cmp(&right.test_date),
        };

        if config.ascending {
            ordering
        } else {
            ordering.reverse()
        }
    });
}

#[function_component(App)]
fn app() -> Html {
    let all_data = use_state(sample_data);
    let filter_text = use_state(String::new);
    let sort_config = use_state(|| None::<SortConfig>);
    let selected_benchmark = use_state(|| None::<Benchmark>);

    let on_filter_input = {
        let filter_text = filter_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            filter_text.set(input.value());
        })
    };

    let on_sort_device = {
        let sort_config = sort_config.clone();
        Callback::from(move |_| {
            sort_config.set(toggle_sort(*sort_config, SortField::Device));
        })
    };

    let on_sort_benchmark = {
        let sort_config = sort_config.clone();
        Callback::from(move |_| {
            sort_config.set(toggle_sort(*sort_config, SortField::Benchmark));
        })
    };

    let on_sort_score = {
        let sort_config = sort_config.clone();
        Callback::from(move |_| {
            sort_config.set(toggle_sort(*sort_config, SortField::Score));
        })
    };

    let on_sort_cost = {
        let sort_config = sort_config.clone();
        Callback::from(move |_| {
            sort_config.set(toggle_sort(*sort_config, SortField::CostPerRun));
        })
    };

    let on_sort_test_date = {
        let sort_config = sort_config.clone();
        Callback::from(move |_| {
            sort_config.set(toggle_sort(*sort_config, SortField::TestDate));
        })
    };

    let on_close_details = {
        let selected_benchmark = selected_benchmark.clone();
        Callback::from(move |_| {
            selected_benchmark.set(None);
        })
    };

    let mut filtered: Vec<Benchmark> = all_data
        .iter()
        .filter(|item| {
            let text = filter_text.to_lowercase();
            item.device.to_lowercase().contains(&text)
                || item.benchmark.to_lowercase().contains(&text)
        })
        .cloned()
        .collect();

    if let Some(config) = *sort_config {
        sort_benchmarks(&mut filtered, config);
    }

    let details_modal = if let Some(benchmark) = (*selected_benchmark).clone() {
        let details = build_benchmark_details(&benchmark);

        html! {
            <div class="details-overlay">
                <section class="details-modal" role="dialog" aria-modal="true" aria-label="Benchmark details">
                    <div class="details-header">
                        <div>
                            <span class="eyebrow">{ "Benchmark details" }</span>
                            <h2>{ &benchmark.benchmark }</h2>
                            <p>
                                { format!(
                                    "{} · score {} · {} per run",
                                    benchmark.device,
                                    benchmark.score,
                                    format_cost(benchmark.cost_per_run_cents)
                                ) }
                            </p>
                        </div>
                        <button class="details-close" type="button" onclick={on_close_details.clone()}>{ "Close" }</button>
                    </div>

                    <div class="detail-grid">
                        <article class="detail-card">
                            <span class="detail-label">{ "Provider" }</span>
                            <strong>{ &details.provider }</strong>
                        </article>
                        <article class="detail-card">
                            <span class="detail-label">{ "Platform architecture" }</span>
                            <strong>{ &details.cpu_arch }</strong>
                        </article>
                        <article class="detail-card">
                            <span class="detail-label">{ "vCPUs" }</span>
                            <strong>{ details.vcpus }</strong>
                        </article>
                        <article class="detail-card">
                            <span class="detail-label">{ "Memory" }</span>
                            <strong>{ format!("{} GiB", details.memory_gib) }</strong>
                        </article>
                        <article class="detail-card">
                            <span class="detail-label">{ "Runtime" }</span>
                            <strong>{ format_duration(details.runtime_secs) }</strong>
                        </article>
                        <article class="detail-card">
                            <span class="detail-label">{ "Hourly cost" }</span>
                            <strong>{ format_cost(details.hourly_cost_cents) }</strong>
                        </article>
                    </div>

                    <div class="detail-columns">
                        <section class="detail-section">
                            <h3>{ "Environment" }</h3>
                            <ul class="detail-list">
                                <li><span>{ "Instance type" }</span><strong>{ &details.instance_type }</strong></li>
                                <li><span>{ "Platform model" }</span><strong>{ &details.cpu_model }</strong></li>
                                <li><span>{ "Region" }</span><strong>{ &details.region }</strong></li>
                                <li><span>{ "OS image" }</span><strong>{ &details.operating_system }</strong></li>
                            </ul>
                        </section>

                        <section class="detail-section">
                            <h3>{ "Run metadata" }</h3>
                            <ul class="detail-list">
                                <li><span>{ "Benchmark focus" }</span><strong>{ &details.benchmark_focus }</strong></li>
                                <li><span>{ "Samples collected" }</span><strong>{ details.samples }</strong></li>
                                <li><span>{ "Test date" }</span><strong>{ &benchmark.test_date }</strong></li>
                                <li><span>{ "Synthetic note" }</span><strong>{ "Generated mock metadata" }</strong></li>
                            </ul>
                        </section>
                    </div>

                    <p class="detail-footnote">{ &details.notes }</p>
                </section>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <main class="app-shell">
            <section class="hero-card">
                <span class="eyebrow">{ "Benchmark Monitor" }</span>
                <h1>{ "Cloud Benchmark Dashboard" }</h1>
                <p>
                    { "Mock cloud benchmark data for dashboard and sorting tests." }
                </p>
            </section>

            <section class="panel">
                <div class="controls">
                    <input
                        type="text"
                        placeholder="Search cloud platforms, instance types, or benchmarks"
                        value={(*filter_text).clone()}
                        oninput={on_filter_input}
                    />
                </div>

                <p class="results-count">{ format!("Showing {} result(s)", filtered.len()) }</p>

                <div class="table-wrap">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    <button class="sort-button" type="button" onclick={on_sort_device}>
                                        { format!("Platform{}", sort_indicator(*sort_config, SortField::Device)) }
                                    </button>
                                </th>
                                <th>
                                    <button class="sort-button" type="button" onclick={on_sort_benchmark}>
                                        { format!("Benchmark{}", sort_indicator(*sort_config, SortField::Benchmark)) }
                                    </button>
                                </th>
                                <th>
                                    <button class="sort-button" type="button" onclick={on_sort_score}>
                                        { format!("Score{}", sort_indicator(*sort_config, SortField::Score)) }
                                    </button>
                                </th>
                                <th>
                                    <button class="sort-button" type="button" onclick={on_sort_cost}>
                                        { format!("Cost / Run{}", sort_indicator(*sort_config, SortField::CostPerRun)) }
                                    </button>
                                </th>
                                <th>
                                    <button class="sort-button" type="button" onclick={on_sort_test_date}>
                                        { format!("Test Date{}", sort_indicator(*sort_config, SortField::TestDate)) }
                                    </button>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            { for filtered.iter().map(|item| html! {
                                <tr>
                                    <td>{ &item.device }</td>
                                    <td>
                                        <button
                                            class="benchmark-link"
                                            type="button"
                                            onclick={{
                                                let selected_benchmark = selected_benchmark.clone();
                                                let item = item.clone();
                                                Callback::from(move |_| {
                                                    selected_benchmark.set(Some(item.clone()));
                                                })
                                            }}
                                        >
                                            { &item.benchmark }
                                        </button>
                                    </td>
                                    <td>{ item.score }</td>
                                    <td>{ format_cost(item.cost_per_run_cents) }</td>
                                    <td>{ &item.test_date }</td>
                                </tr>
                            })}
                        </tbody>
                    </table>
                </div>
            </section>

            { details_modal }
        </main>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(test)]
mod tests;
