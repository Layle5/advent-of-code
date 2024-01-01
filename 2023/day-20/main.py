import copy
import dataclasses
import enum
import itertools
import math

from typing import List, Tuple


def main():
    with open('input.txt', 'r') as file:
        lines = map(str.strip, file)
        modules = list(map(parse_module, lines))
        modules_dict = {m.name: m for m in modules}
        for module in modules:
            for destination_name in module.destinations_names:
                destination_module = modules_dict.get(destination_name)
                if destination_module is not None:
                    destination_module.sources_names.append(module.name)

    part_one(copy.deepcopy(modules_dict))
    part_two(modules_dict)


def part_one(modules_dict):
    low_total, high_total = 0, 0
    for _ in range(1000):
        low_count, high_count, _ = cycle(modules_dict)
        low_total += low_count
        high_total += high_count
    print(f"Part 1: {low_total * high_total}")


def part_two(modules_dict):
    semi_final_layer = list(
        itertools.chain(*[m.sources_names for m in modules_dict.values() if 'rx' in m.destinations_names]))
    semi_final_layer_cycles = {n: None for n in semi_final_layer}
    for cycle_number in itertools.count(1):
        _, _, high_from_conjunctions = cycle(modules_dict, cycle_per_conjunctions=semi_final_layer_cycles)
        for conjunction in high_from_conjunctions:
            if conjunction in semi_final_layer_cycles:
                semi_final_layer_cycles[conjunction] = semi_final_layer_cycles[conjunction] or cycle_number
        if all(semi_final_layer_cycles.values()):
            break

    print(f"Part 2: {math.lcm(*semi_final_layer_cycles.values())}")


@dataclasses.dataclass
class Signal:
    source_name: str
    is_high: bool
    destination_name: str


class ModuleType(enum.StrEnum):
    FLIP_FLOP = '%'
    CONJUNCTION = '&'
    BROADCASTER = 'broadcaster'


@dataclasses.dataclass
class BaseModule:
    name: str
    sources_names: List[str]
    destinations_names: List[str]


@dataclasses.dataclass
class FlipFlopModule(BaseModule):
    is_on: bool = False


@dataclasses.dataclass
class ConjunctionModule(BaseModule):
    received: dict[str, bool] = dataclasses.field(default_factory=dict)


@dataclasses.dataclass
class BroadcasterModule(BaseModule):
    pass


def cycle(modules_dict: dict[str, BaseModule], cycle_per_conjunctions=None) -> Tuple[int, int, set[str]]:
    low_count, high_count = 0, 0
    signals = [Signal('button', False, 'broadcaster')]
    high_from_conjunctions = set()

    while signals:
        new_signals = []

        for signal in signals:
            if signal.is_high:
                high_count += 1
            else:
                low_count += 1

            module = modules_dict.get(signal.destination_name)
            if module is None:
                continue

            def send_signals(is_high: bool):
                new_signals.extend(map(lambda n: Signal(module.name, is_high, n), module.destinations_names))

            if isinstance(module, BroadcasterModule):
                send_signals(False)
            elif isinstance(module, FlipFlopModule):
                if not signal.is_high:
                    module.is_on = not module.is_on
                    send_signals(module.is_on)
            elif isinstance(module, ConjunctionModule):
                module.received[signal.source_name] = signal.is_high
                new_signal = not all(module.received.get(s) or False for s in module.sources_names)
                send_signals(new_signal)
                if cycle_per_conjunctions is not None and new_signal and module.name in cycle_per_conjunctions:
                    high_from_conjunctions.add(module.name)

        signals = new_signals

    return low_count, high_count, high_from_conjunctions


def parse_module(line: str) -> BaseModule:
    name_type, destinations = line.split(' -> ')
    module_name, module_type = (
        (name_type, BroadcasterModule) if name_type == ModuleType.BROADCASTER else
        (name_type[1:], FlipFlopModule) if name_type.startswith(ModuleType.FLIP_FLOP) else
        (name_type[1:], ConjunctionModule)
    )
    destinations_names = destinations.split(', ')
    return module_type(name=module_name, sources_names=[], destinations_names=destinations_names)


if __name__ == '__main__':
    main()
