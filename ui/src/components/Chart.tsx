import { createSignal, onCleanup, onMount, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

export type ChartProps = {
  plot: Plot.PlotOptions;
  loading: boolean;
};

export const Chart = (props: ChartProps) => {
  const [width, setWidth] = createSignal<{ width: number; height: number }>({
    height: window.innerHeight,
    width: window.innerWidth,
  });

  const handler = (_event: Event) => {
    setWidth({ height: window.innerHeight, width: window.innerWidth });
  };

  onMount(() => {
    window.addEventListener("resize", handler);
  });

  onCleanup(() => {
    window.removeEventListener("resize", handler);
  });

  return (
    <div class="w-full">
      <Show when={!props.loading}>
        {Plot.plot({ width: width().width, ...props.plot })}
      </Show>
    </div>
  );
};
