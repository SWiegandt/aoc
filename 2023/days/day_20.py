import math
from abc import abstractmethod
from collections import defaultdict
from typing import List

from day import Day


class Module:
    low = 0
    high = 0
    rx_cycles = {}

    def __init__(self, name: str) -> None:
        self.outputs: List["Module"] = []
        self.name = name

    def add_output(self, module: "Module"):
        self.outputs.append(module)

    @abstractmethod
    def receive(self, pulse: int, sender: "Module", schedule: list):
        pass

    def emit(self, pulse: int, schedule: list):
        for output in self.outputs:
            if output.name == "rx":
                for module, state in self.inputs.items():
                    if not state:
                        continue

                    Module.rx_cycles[module] = Button.presses

                if all(module in Module.rx_cycles for module in self.inputs.keys()):
                    raise Exception(math.prod(Module.rx_cycles.values()))

            if pulse:
                Module.high += 1
            else:
                Module.low += 1

            schedule.append((lambda output: lambda: output.receive(pulse, self, schedule))(output))

    @staticmethod
    def get(module):
        if module.startswith("%") or module.startswith("&"):
            module_name = module[1:]
        else:
            module_name = module

        if module == "broadcaster":
            return module_name, Broadcaster(module_name)
        elif module.startswith("%"):
            return module_name, FlipFlop(module_name)
        elif module.startswith("&"):
            return module_name, Conjunction(module_name)

        return module_name, NoneModule()

    @staticmethod
    def reset():
        Module.low = Module.high = 0
        Module.rx_cycles = defaultdict(list)

    def __str__(self) -> str:
        return f"{self.name}"


class FlipFlop(Module):
    def __init__(self, name: str) -> None:
        super().__init__(name)
        self.state = False

    def receive(self, pulse: int, sender: Module, schedule: list):
        if not pulse:
            self.state = not self.state
            self.emit(int(self.state), schedule)

    def __str__(self) -> str:
        return f"{self.name}({self.state})"


class Conjunction(Module):
    def __init__(self, name: str) -> None:
        super().__init__(name)
        self.inputs = {}

    def add_input(self, input: Module):
        self.inputs[input] = 0

    def receive(self, pulse: int, sender: Module, schedule: list):
        self.inputs[sender] = pulse
        self.emit(int(not all(self.inputs.values())), schedule)

    def __str__(self) -> str:
        return f"{self.name}({','.join(f'{k.name}={v}' for k, v in self.inputs.items())})"


class Broadcaster(Module):
    def receive(self, pulse: int, sender: "Module", schedule: list):
        self.emit(pulse, schedule)


class Button(Module):
    presses = 0

    def __init__(self, broadcaster) -> None:
        super().__init__("button")
        self.add_output(broadcaster)
        Button.presses = 0

    def press(self):
        Module.low += 1
        Button.presses += 1
        schedule = []
        self.outputs[0].receive(0, self, schedule)

        while schedule:
            schedule.pop(0)()


class NoneModule(Module):
    pass


class DayTwenty(Day):
    day = 20

    def get_button(self, input):
        Module.reset()
        module_names, outputs = zip(*(row.split(" -> ") for row in input))
        modules = {}

        for module in module_names:
            modules.update([Module.get(module)])

        for module, outputs in zip(modules.values(), outputs):
            for output in outputs.split(", "):
                module.add_output(modules.get(output, NoneModule(output)))

                if isinstance(modules.get(output), Conjunction):
                    modules[output].add_input(module)

        return Button(modules["broadcaster"])

    def one(self, input):
        button = self.get_button(input)

        for _ in range(1000):
            button.press()

        return Module.low * Module.high

    def two(self, input):
        button = self.get_button(input)

        while True:
            try:
                button.press()
            except Exception as e:
                return e.args[0]


if __name__ == "__main__":
    DayTwenty().run()
