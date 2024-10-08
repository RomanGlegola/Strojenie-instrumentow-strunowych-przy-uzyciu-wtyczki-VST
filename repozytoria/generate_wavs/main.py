"""
This module provides a command-line interface to run a Rust program with
various audio scenarios.

The script accepts a specified scenario and the path to a Rust executable,
and executes the Rust program with all combinations of settings defined for
the selected scenario. It uses the `argparse` module for command-line argument
parsing and the `subprocess` module to execute the Rust program.

Functions:
    -run_rust_program(executable_path: str, **kwargs: Any) -> None:
        Executes the Rust program with the provided executable path and
            keyword arguments representing the audio settings.

    -execute_scenario(executable_path: str, settings: Dict[str, list]) -> None:
        Generates all combinations of the provided scenario settings and runs
            the Rust program for each combination.

    -main() -> None:
        The entry point of the script, which sets up argument parsing and
            invokes the appropriate functions to run the scenario.

Command-line Arguments:
    -scenario:
        The name of the audio scenario to run, chosen from the available
            scenarios defined in the SCENARIOS constant.

    -executable_path:
        The path to the compiled Rust executable that will be executed with
            the specified scenario settings.
"""

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
