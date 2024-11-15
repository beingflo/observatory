import { For } from "solid-js";
import { useRange } from "./RangeProvider";

export const DateRangeSelector = () => {
  const [_, { setFrom }] = useRange();

  const options = ["1y", "6m", "30d", "7d", "1d", "6h"];

  const getDate = (label: string): string => {
    const now = new Date();
    switch (label) {
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

  return (
    <div class="h-fit flex flex-row gap-1">
      <For each={options}>
        {(item) => (
          <button
            onClick={() => setFrom(getDate(item))}
            class="text-sm px-3 text-gray-600 rounded-sm"
          >
            {item}
          </button>
        )}
      </For>
    </div>
  );
};
