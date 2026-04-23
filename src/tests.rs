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

    assert_eq!(data.first().map(|item| item.test_date.as_str()), Some("2026-03-25"));
    assert_eq!(data.last().map(|item| item.test_date.as_str()), Some("2026-02-03"));
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