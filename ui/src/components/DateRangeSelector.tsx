import { For, Show, untrack } from "solid-js";
import { useRange } from "./RangeProvider";
import { createShortcut } from "@solid-primitives/keyboard";

export const getDate = (option: string): string => {
  const now = new Date();
  switch (option) {
    case "1y":
      now.setMonth(now.getMonth() - 12);
      return now.toISOString();
    case "6m":
      now.setMonth(now.getMonth() - 6);
      return now.toISOString();
    case "30d":
      now.setMonth(now.getMonth() - 1);
      return now.toISOString();
    case "7d":
      now.setHours(now.getHours() - 168);
      return now.toISOString();
    case "1d":
      now.setHours(now.getHours() - 24);
      return now.toISOString();
    case "6h":
      now.setHours(now.getHours() - 6);
      return now.toISOString();
  }

  return "";
};

export const DateRangeSelector = () => {
  const [{ fromOption, from, to }, { setFromOption, setFrom, setTo }] =
    useRange();

  const options = ["1y", "6m", "30d", "7d", "1d", "6h", "C"];

  createShortcut(["ArrowLeft"], () => {
    // TODO
    console.log("Move time window left");
  });
  createShortcut(["ArrowRight"], () => {
    // TODO
    console.log("Move time window right");
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
        <div class="flex gap-4">
          <input
            class="text-sm font-light text-center col-span-1"
            type="datetime-local"
            name="date"
            onInput={(event) =>
              setFrom(new Date(event?.currentTarget.value).toISOString())
            }
            value={untrack(() => from()?.split(".")[0])}
          />
          <input
            class="text-sm font-light text-center col-span-1"
            type="datetime-local"
            name="date"
            onInput={(event) =>
              setTo(new Date(event?.currentTarget.value).toISOString())
            }
            value={untrack(() => to()?.split(".")[0])}
          />
        </div>
      </Show>
    </div>
  );
};
