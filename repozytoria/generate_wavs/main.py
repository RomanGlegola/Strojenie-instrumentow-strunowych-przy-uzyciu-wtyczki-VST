"""
This module provides a command-line interface to run a Rust program with
various audio scenarios.

The script accepts a specified scenario and the path to a Rust executable,
and executes the Rust program with all combinations of settings defined for
the selected scenario. It uses the `argparse` module for command-line argument
parsing and the `subprocess` module to execute the Rust program.

Functions
---------
- :func:`run_rust_program`: Executes the Rust program with the provided
  executable path and keyword arguments representing the audio settings.
- :func:`execute_scenario`: Generates all combinations of the provided scenario
  settings and runs the Rust program for each combination.
- :func:`main`: The entry point of the script, which sets up argument parsing
  and invokes the appropriate functions to run the scenario.

Parameters
----------
:arg str scenario:
    The name of the audio scenario to run, chosen from the available scenarios
    defined in the `SCENARIOS` constant.
:arg str executable_path:
    The path to the compiled Rust executable that will be executed with the
    specified scenario settings.
"""

import argparse
import subprocess
from itertools import product
from typing import Any, Union

from constants.sound_scenario import SCENARIOS


def run_rust_program(executable_path: str, **kwargs: Any) -> None:
    """
    Execute a Rust program with specified command-line arguments.

    This function constructs a command to run a Rust executable located at the
    given path, appending the provided keyword arguments as command-line
    options.

    :param str executable_path: The file path to the compiled Rust executable.
    :param **kwargs:
        Arbitrary keyword arguments representing command-line options
            and their values. Each key will be prefixed with '--' when
            passed to the Rust program.
    :raises subprocess.CalledProcessError:
        If the Rust program exits with a non-zero status.
    :rtype: None
    """
    args: list[str] = [executable_path]

    for key, value in kwargs.items():
        args.append(f"--{key}")
        args.append(str(value))
    subprocess.run(args=args, check=True)


def execute_scenario(
    executable_path: str,
    settings: dict[str, tuple[Union[str, int, float], ...]],
) -> None:
    """
    Execute the Rust program with all combinations of settings for a specified
    scenario.

    This function generates all possible combinations of the provided settings
    and runs the Rust program for each combination. The settings are unpacked
    from a dictionary, and each combination is passed as keyword arguments to
    the run_rust_program function.

    :param str executable_path:
        The file path to the compiled Rust executable.
    :param Dict[str, list] settings:
        A dictionary where keys represent setting names and values are lists
        of possible values for each setting.
    :raises ValueError:
        If the settings dictionary is empty or if there are no values for
            any key.
    :rtype: None
    """
    keys, values = zip(*settings.items())
    for combination in product(*values):
        combination_dict = dict(zip(keys, combination))
        run_rust_program(executable_path, **combination_dict)


def main() -> None:
    """
    Entry point for the script to run a Rust program with specified audio
    scenarios.

    This function sets up the command-line argument parser, defining required
    arguments for the scenario and the path to the Rust executable. It
    retrieves the selected scenario's settings and invokes the execute_scenario
    function to run the Rust program with the appropriate configurations.

    :arg scenario:
        The name of the audio scenario to be executed, chosen from the
            available scenarios defined in the SCENARIOS constant.
    :arg executable_path:
        The path to the compiled Rust executable that will be run with the
            selected scenario settings.

    :raises SystemExit:
        If the provided arguments are invalid or if there are issues during
            the execution of the Rust program.
    """
    parser: argparse.ArgumentParser = argparse.ArgumentParser(
        description="Run the Rust program with different scenarios"
    )
    parser.add_argument(
        "scenario", choices=SCENARIOS.keys(), help="Choose a scenario"
    )
    parser.add_argument(
        "executable_path", help="Path to the compiled Rust executable"
    )
    args: argparse.Namespace = parser.parse_args()

    scenario_settings: dict[str, tuple[Union[str, int, float], ...]] = (
        SCENARIOS[args.scenario]
    )
    execute_scenario(
        executable_path=args.executable_path, settings=scenario_settings
    )


if __name__ == "__main__":
    main()
