import { createSignal, onCleanup, onMount, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

export type ChartProps = {
  plot: Plot.PlotOptions;
  loading: boolean;
  id: string;
};

export const Chart = (props: ChartProps) => {
  const [width, setWidth] = createSignal(0);

  const onResize = (entries: ResizeObserverEntry[]) => {
    setWidth(entries[0].contentRect.width);
  };

  const observer = new ResizeObserver(onResize);

  onMount(() => {
    observer.observe(document.getElementById(props.id)!);
  });

  onCleanup(() => {
    observer.disconnect();
  });

  return (
    <div id={props.id} class="w-full">
      <Show when={!props.loading}>
        {Plot.plot({ width: width(), ...props.plot })}
      </Show>
    </div>
  );
};
