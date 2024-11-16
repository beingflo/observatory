import { createSignal, onCleanup, onMount, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

export type ChartProps = {
  plot: Plot.PlotOptions;
  loading: boolean;
  id: string;
};

export const Chart = (props: ChartProps) => {
  const [width, setWidth] = createSignal(0);
  const [height, setHeight] = createSignal(0);

  const onResize = (entries: ResizeObserverEntry[]) => {
    setWidth(entries[0].contentRect.width);
    setHeight(entries[0].contentRect.height);
  };

  const observer = new ResizeObserver(onResize);

  onMount(() => {
    observer.observe(document.getElementById(props.id)!);
  });

  onCleanup(() => {
    observer.disconnect();
  });

  return (
    <div id={props.id} class="w-full h-80">
      <Show
        when={!props.loading}
        fallback={
          <div class="h-full flex flex-row items-center justify-center">
            <p class="w-fit">...</p>
          </div>
        }
      >
        {Plot.plot({ width: width(), height: height(), ...props.plot })}
      </Show>
    </div>
  );
};
