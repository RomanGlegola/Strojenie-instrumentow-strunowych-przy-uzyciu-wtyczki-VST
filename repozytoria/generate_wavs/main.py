import argparse
import subprocess
from itertools import product
from typing import Any, Dict

from constants.sound_scenario import SCENARIOS


def run_rust_program(executable_path: str, **kwargs: Any) -> None:
    """Run the Rust executable with the given arguments."""
    args = [executable_path]

    for key, value in kwargs.items():
        args.append(__object=f"--{key}")
        args.append(__object=str(value))
    subprocess.run(args=args, check=True)


def execute_scenario(executable_path: str, settings: Dict[str, list]) -> None:
    """Execute the Rust program with different settings combinations."""
    keys, values = zip(*settings.items())
    for combination in product(*values):
        combination_dict = dict(zip(keys, combination))
        run_rust_program(executable_path, **combination_dict)


def main() -> None:
    """Main entry point for the script."""
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
    execute_scenario(
        executable_path=args.executable_path, settings=scenario_settings
    )


if __name__ == "__main__":
    main()
