use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Benchmark {
    device: String,
    benchmark: String,
    score: u32,
    power_watts: u32,
    test_date: String,
}

fn sample_data() -> Vec<Benchmark> {
    vec![
        Benchmark {
            device: "Device A".into(),
            benchmark: "Geekbench".into(),
            score: 1240,
            power_watts: 65,
            test_date: "2026-01-10".into(),
        },
        Benchmark {
            device: "Device B".into(),
            benchmark: "Geekbench".into(),
            score: 1415,
            power_watts: 72,
            test_date: "2026-01-12".into(),
        },
        Benchmark {
            device: "Device C".into(),
            benchmark: "Cinebench".into(),
            score: 980,
            power_watts: 54,
            test_date: "2026-01-15".into(),
        },
        Benchmark {
            device: "Device D".into(),
            benchmark: "Cinebench".into(),
            score: 1630,
            power_watts: 88,
            test_date: "2026-01-18".into(),
        },
        Benchmark {
            device: "Device E".into(),
            benchmark: "Speedometer".into(),
            score: 1120,
            power_watts: 48,
            test_date: "2026-01-20".into(),
        },
    ]
}

#[function_component(App)]
fn app() -> Html {
    let all_data = use_state(sample_data);
    let filter_text = use_state(String::new);
    let min_score = use_state(|| 0u32);

    let on_filter_input = {
        let filter_text = filter_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            filter_text.set(input.value());
        })
    };

    let on_min_score_input = {
        let min_score = min_score.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let parsed = input.value().parse::<u32>().unwrap_or(0);
            min_score.set(parsed);
        })
    };

    let filtered: Vec<Benchmark> = all_data
        .iter()
        .filter(|item| {
            let text = filter_text.to_lowercase();
            let matches_text = item.device.to_lowercase().contains(&text)
                || item.benchmark.to_lowercase().contains(&text);
            let matches_score = item.score >= *min_score;
            matches_text && matches_score
        })
        .cloned()
        .collect();

    html! {
        <main class="app-shell">
            <section class="hero-card">
                <span class="eyebrow">{ "Benchmark Monitor" }</span>
                <h1>{ "Synthetic Benchmark Dashboard" }</h1>
                <p>
                    { "Demo application for CI/CD deployment. All benchmark data shown here is synthetic." }
                </p>
            </section>

            <section class="panel">
                <div class="controls">
                    <input
                        type="text"
                        placeholder="Filter by device or benchmark"
                        value={(*filter_text).clone()}
                        oninput={on_filter_input}
                    />
                    <input
                        type="number"
                        placeholder="Minimum score"
                        value={(*min_score).to_string()}
                        oninput={on_min_score_input}
                    />
                </div>

                <p class="results-count">{ format!("Showing {} result(s)", filtered.len()) }</p>

                <div class="table-wrap">
                    <table>
                        <thead>
                            <tr>
                                <th>{ "Device" }</th>
                                <th>{ "Benchmark" }</th>
                                <th>{ "Score" }</th>
                                <th>{ "Power (W)" }</th>
                                <th>{ "Test Date" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for filtered.iter().map(|item| html! {
                                <tr>
                                    <td>{ &item.device }</td>
                                    <td>{ &item.benchmark }</td>
                                    <td>{ item.score }</td>
                                    <td>{ item.power_watts }</td>
                                    <td>{ &item.test_date }</td>
                                </tr>
                            })}
                        </tbody>
                    </table>
                </div>
            </section>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
