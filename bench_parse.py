import json
import sys
from pathlib import Path


def main() -> None:
    tmpfile = Path(sys.argv[1])
    case_name = sys.argv[2]
    tool_names = sys.argv[3:]

    data = json.loads(tmpfile.read_text())

    times: dict[str, dict[str, float]] = {}
    for i, result in enumerate(data["results"]):
        if i < len(tool_names):
            times[tool_names[i]] = {
                "mean": result["mean"],
                "stddev": result["stddev"],
            }

    print(json.dumps({"case": case_name, "results": times}, indent=2))


if __name__ == "__main__":
    main()
