import { For } from "solid-js";
import { useRange } from "./RangeProvider";

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
  const [{ fromOption }, { setFromOption }] = useRange();

  const options = ["1y", "6m", "30d", "7d", "1d", "6h"];

  return (
    <div class="h-fit flex flex-row gap-2">
      <For each={options}>
        {(item) => (
          <button onClick={() => setFromOption(item)}>
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
  );
};
