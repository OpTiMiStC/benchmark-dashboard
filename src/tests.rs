use super::*;

#[test]
fn sample_data_contains_fifty_entries() {
    let data = sample_data();

    assert_eq!(data.len(), 50);
    assert!(data
        .iter()
        .all(|item| item.device.starts_with("AWS") || item.device.starts_with("GCP")));
}

#[test]
fn format_cost_uses_dollars_and_cents() {
    assert_eq!(format_cost(68), "$0.68");
    assert_eq!(format_cost(402), "$4.02");
}

#[test]
fn format_duration_formats_minutes_and_seconds() {
    assert_eq!(format_duration(45), "45s");
    assert_eq!(format_duration(300), "5m 0s");
}

#[test]
fn toggle_sort_flips_same_field_direction() {
    let initial = toggle_sort(None, SortField::Score);
    let toggled = toggle_sort(initial, SortField::Score);
    let switched = toggle_sort(toggled, SortField::Benchmark);

    assert_eq!(
        initial,
        Some(SortConfig {
            field: SortField::Score,
            ascending: true,
        })
    );
    assert_eq!(
        toggled,
        Some(SortConfig {
            field: SortField::Score,
            ascending: false,
        })
    );
    assert_eq!(
        switched,
        Some(SortConfig {
            field: SortField::Benchmark,
            ascending: true,
        })
    );
}

#[test]
fn sort_benchmarks_orders_by_cost_and_date() {
    let mut data = sample_data();

    sort_benchmarks(
        &mut data,
        SortConfig {
            field: SortField::CostPerRun,
            ascending: true,
        },
    );

    assert_eq!(data.first().map(|item| item.cost_per_run_cents), Some(56));
    assert_eq!(data.last().map(|item| item.cost_per_run_cents), Some(402));

    sort_benchmarks(
        &mut data,
        SortConfig {
            field: SortField::TestDate,
            ascending: false,
        },
    );

    assert_eq!(
        data.first().map(|item| item.test_date.as_str()),
        Some("2026-03-25")
    );
    assert_eq!(
        data.last().map(|item| item.test_date.as_str()),
        Some("2026-02-03")
    );
}

#[test]
fn sort_indicator_matches_active_direction() {
    let ascending = Some(SortConfig {
        field: SortField::Device,
        ascending: true,
    });
    let descending = Some(SortConfig {
        field: SortField::Device,
        ascending: false,
    });

    assert_eq!(sort_indicator(ascending, SortField::Device), " ↑");
    assert_eq!(sort_indicator(descending, SortField::Device), " ↓");
    assert_eq!(sort_indicator(descending, SortField::Score), "");
}

#[test]
fn build_benchmark_details_generates_drill_down_metadata() {
    let benchmark = Benchmark {
        device: "AWS c8g.2xlarge (Graviton4)".into(),
        benchmark: "Geekbench 6 Multi".into(),
        score: 70210,
        cost_per_run_cents: 138,
        test_date: "2026-02-05".into(),
    };

    let details = build_benchmark_details(&benchmark);

    assert_eq!(details.provider, "AWS");
    assert_eq!(details.instance_type, "c8g.2xlarge");
    assert_eq!(details.cpu_model, "Graviton4");
    assert_eq!(details.cpu_arch, "Arm64");
    assert_eq!(details.vcpus, 8);
    assert_eq!(details.memory_gib, 16);
    assert_eq!(details.runtime_secs, 300);
    assert_eq!(details.hourly_cost_cents, 1656);
}

#[test]
fn estimate_vcpus_supports_gcp_shapes() {
    assert_eq!(estimate_vcpus("GCP", "c3-highcpu-44"), 44);
    assert_eq!(estimate_vcpus("GCP", "t2a-standard-16"), 16);
}
