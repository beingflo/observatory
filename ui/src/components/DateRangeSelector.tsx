import { createSignal, For, Show } from "solid-js";
import { useRange } from "./RangeProvider";
import { createShortcut } from "@solid-primitives/keyboard";

const getSecondsFromOption = (option: string): number => {
  switch (option) {
    case "1y":
      return 3600 * 24 * 30 * 12;
    case "6m":
      return 3600 * 24 * 30 * 6;
    case "30d":
      return 3600 * 24 * 30;
    case "7d":
      return 3600 * 24 * 7;
    case "1d":
      return 3600 * 24;
    case "6h":
      return 3600 * 6;
  }
  return 0;
};

export const getDate = (option: string, startDate?: Date): string => {
  const start = startDate || new Date();
  start.setSeconds(start.getSeconds() - getSecondsFromOption(option));

  return start.toISOString();
};

const trimISOString = (value: string) => value?.split(".")[0];

export const DateRangeSelector = () => {
  const [{ fromOption, from, to }, { setFromOption, setFrom, setTo }] =
    useRange();

  const [fromValue, setFromValue] = createSignal(trimISOString(from()));
  const [toValue, setToValue] = createSignal(trimISOString(to()));

  const options = ["1y", "6m", "30d", "7d", "1d", "6h", "C"];

  createShortcut(["ArrowLeft"], () => {
    const oldTo = new Date(to());
    const oldFrom = new Date(from());

    let newFrom = new Date();
    newFrom.setTime(oldFrom.getTime() - (oldTo.getTime() - oldFrom.getTime()));

    setFromOption("C");
    setTo(oldFrom.toISOString());
    setFrom(newFrom.toISOString());

    setFromValue(trimISOString(newFrom.toISOString()));
    setToValue(trimISOString(oldFrom.toISOString()));
  });
  createShortcut(["ArrowRight"], () => {
    const oldTo = new Date(to());
    const oldFrom = new Date(from());

    let newTo = new Date();
    newTo.setTime(oldTo.getTime() + (oldTo.getTime() - oldFrom.getTime()));

    setFromOption("C");
    setTo(newTo.toISOString());
    setFrom(oldTo.toISOString());

    setFromValue(trimISOString(oldTo.toISOString()));
    setToValue(trimISOString(newTo.toISOString()));
  });

  return (
    <div class="w-full flex flex-col items-start md:items-end">
      <div class="h-fit flex flex-row gap-2">
        <For each={options}>
          {(item) => (
            <button
              onClick={() => {
                setFromOption(item);
                setTo(new Date().toISOString());
              }}
            >
              <div
                class={`text-sm px-2 pb-0.5 flex justify-center items-center text-gray-600 rounded-sm ${
                  fromOption() === item && "bg-white border border-black"
                }`}
              >
                {item}
              </div>
            </button>
          )}
        </For>
      </div>
      <Show when={fromOption() === "C"}>
        <div class="flex flex-col md:flex-row mt-2 gap-2 md:gap-4">
          <input
            class="text-sm font-light text-center col-span-1"
            type="datetime-local"
            name="date"
            onInput={(event) =>
              setFrom(new Date(event?.currentTarget.value).toISOString())
            }
            value={fromValue()}
          />
          <input
            class="text-sm font-light text-center col-span-1"
            type="datetime-local"
            name="date"
            onInput={(event) =>
              setTo(new Date(event?.currentTarget.value).toISOString())
            }
            value={toValue()}
          />
        </div>
      </Show>
    </div>
  );
};
