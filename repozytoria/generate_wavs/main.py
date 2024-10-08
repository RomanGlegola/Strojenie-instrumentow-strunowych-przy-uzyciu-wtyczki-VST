import argparse
import subprocess
from itertools import product

from data.sound_scenario import SCENARIOS


def run_rust_program(executable_path, **kwargs):
    args = [executable_path]

    for key, value in kwargs.items():
        args.append(f"--{key}")
        args.append(str(value))
    subprocess.run(args, check=True)


def execute_scenario(executable_path, settings):
    keys, values = zip(*settings.items())
    for combination in product(*values):
        combination_dict = dict(zip(keys, combination))
        run_rust_program(executable_path, **combination_dict)


def main():
    parser = argparse.ArgumentParser(
        description="Run the Rust program with different scenarios"
    )
    parser.add_argument(
        "scenario", choices=SCENARIOS.keys(), help="Choose a scenario"
    )
    parser.add_argument(
        "executable_path", help="Path to the compiled Rust executable"
    )
    args = parser.parse_args()

    scenario_settings = SCENARIOS[args.scenario]
    execute_scenario(args.executable_path, scenario_settings)


if __name__ == "__main__":
    main()
