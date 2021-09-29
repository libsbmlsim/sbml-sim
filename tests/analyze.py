from rich.console import Console
import typer


def main(test_results: str):

    results = {}
    # read result file
    with open(test_results, 'r') as f:
        for line in f.readlines():
            result, test_name = line.strip().split(' ')
            test_n = [int(s) for s in test_name.split('_') if s.isdigit()][0]
            results[test_n] = result

    tagMetrics = {
        "componentTags": {},
        "testTags": {}
    }

    for test_n, result in results.items():
        description = get_model_description(test_n)
        tagMetrics = refresh_tag_metrics(tagMetrics, description, result)

    printMetrics(tagMetrics)


def get_model_description(test_n):
    description_filename = f"../testsuites/core-semantic/{test_n:05}/{test_n:05}-model.m"

    description = {
        "test_number": test_n,
        "componentTags": [],
        "testTags": [],
        "levels": [],
        "packagesPresent": []
    }

    with open(description_filename, 'r') as file:
        for line in file.readlines():
            for key in description.keys():
                if key + ":" in line:
                    line = line.split(':')[1]
                    items = line.strip().split(',')
                    description[key] = [x.strip()
                                        for x in items]
                    if '' in description[key]:
                        description[key].remove('')

    return description


def refresh_tag_metrics(metrics, current_model_desc, current_result):
    if current_result == "ignored":
        return metrics
    for tags in ["componentTags", "testTags"]:
        for tag in current_model_desc[tags]:
            if tag not in metrics[tags].keys():
                metrics[tags][tag] = {"pass": 0, "fail": 0, "skip": 0}

            if current_result == "ok":
                metrics[tags][tag]["pass"] += 1
            elif current_result == "failed":
                metrics[tags][tag]["fail"] += 1
            elif current_result == "ignored":
                metrics[tags][tag]["skip"] += 1
            else:
                print(current_model_desc, current_result)
                exit()

            if "CSymbol" in tag:
                print(current_model_desc, current_result)

    return metrics


def printMetrics(metrics):
    console = Console()
    for tags in ["componentTags", "testTags"]:
        console.print(f"[blue]{tags}")
        tags_results = []
        tags_results_pass_percentage = []
        for tag, results in metrics[tags].items():
            pass_percent = results["pass"] / \
                (results["pass"] + results["fail"] + results["skip"]) * 100
            pass_percent = int(pass_percent * 1000) / 1000
            tags_results.append((tag, results, pass_percent))
            tags_results_pass_percentage.append(pass_percent)
        tags_results.sort(key=lambda x: x[2])
        for tag, results, pass_percent in tags_results:
            Pass = results['pass']
            fail = results['fail']
            skip = results['skip']

            console.print(
                f"{tag:30}[green]{Pass:4}  [yellow]{skip:4}  [red]{fail:4}  [green]{pass_percent:7}")


if __name__ == "__main__":
    typer.run(main)
