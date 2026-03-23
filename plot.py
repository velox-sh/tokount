from json import loads
from pathlib import Path
from re import sub
from sys import argv

from matplotlib import pyplot as plt
from matplotlib.ticker import FuncFormatter
import numpy as np


RESULTS_PATH = Path(argv[1]) if len(argv) > 1 else Path("assets/bench_results.json")
OUTPUT_DIR = Path("assets/benchmarks")

FASTEST_COLOR = "#2496ED"
SLOWER_COLOR = "#aaaaaa"


def load_cases(path: Path) -> list[dict]:
    return loads(path.read_text())


def ordered_tools(cases: list[dict]) -> list[str]:
    seen: list[str] = []
    for case in cases:
        for tool in case["results"]:
            if tool not in seen:
                seen.append(tool)
    return sorted(seen, key=lambda t: (t != "tokount", t))


def build_matrices(cases: list[dict], tools: list[str]) -> np.ndarray:
    n, m = len(cases), len(tools)
    means = np.zeros((n, m))
    for i, case in enumerate(cases):
        for j, tool in enumerate(tools):
            if tool in case["results"]:
                means[i, j] = case["results"][tool]["mean"] * 1000  # seconds -> ms
    return means


def fmt_time(ms: float) -> str:
    if ms < 1000:
        return f"{ms:.0f}ms"
    s = ms / 1000
    if s < 60:
        return f"{s:.1f}s"
    return f"{s / 60:.1f}min"


def slugify(name: str) -> str:
    # keep filenames stable and shell-safe across repo names/paths
    return sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def render_case(
    case: dict, tools: list[str], row: np.ndarray, output_dir: Path
) -> Path:
    order = np.argsort(row)[::-1]
    sorted_tools = [tools[i] for i in order]
    sorted_vals = row[order]

    fastest = sorted_vals[sorted_vals > 0].min() if (sorted_vals > 0).any() else 1.0
    colors = [FASTEST_COLOR if v == fastest else SLOWER_COLOR for v in sorted_vals]

    fig, ax = plt.subplots(figsize=(6.8, 3.6))
    fig.patch.set_facecolor("white")

    y = np.arange(len(sorted_tools))
    bars = ax.barh(y, sorted_vals, height=0.6, color=colors, zorder=3)

    x_max = float(sorted_vals.max()) if sorted_vals.max() > 0 else 1.0
    for bar, v in zip(bars, sorted_vals):
        if v <= 0:
            continue
        ax.text(
            v + x_max * 0.02,
            bar.get_y() + bar.get_height() / 2,
            fmt_time(v),
            ha="left",
            va="center",
            fontsize=9,
            color="#333333",
        )

    ax.set_xlim(0, x_max * 1.35)
    ax.set_facecolor("white")
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color("#cccccc")
    ax.spines["bottom"].set_color("#cccccc")
    ax.tick_params(colors="#555555", labelsize=9)
    ax.xaxis.set_major_formatter(FuncFormatter(lambda v, _: fmt_time(v)))
    ax.grid(axis="x", color="#eeeeee", linewidth=0.8, zorder=0)
    ax.set_yticks(y)
    ax.set_yticklabels(sorted_tools, fontsize=10, color="#333333")

    repo_name = str(case["case"])
    ax.set_title(
        f"{repo_name}\nWall-clock benchmark (lower is better)",
        fontsize=12,
        color="#222222",
        pad=8,
    )

    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / f"{slugify(repo_name)}.png"
    fig.savefig(output_path, dpi=170, facecolor="white", bbox_inches="tight")
    plt.close(fig)
    return output_path


def render(cases: list[dict], tools: list[str], means: np.ndarray) -> None:
    for idx, case in enumerate(cases):
        path = render_case(case, tools, means[idx], OUTPUT_DIR)
        print(f"Chart saved: {path}")


def main() -> None:
    cases = load_cases(RESULTS_PATH)
    tools = ordered_tools(cases)
    means = build_matrices(cases, tools)
    render(cases, tools, means)


if __name__ == "__main__":
    main()
