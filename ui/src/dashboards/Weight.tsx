import { createResource, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

const fetchWeight = async () => {
  const response = await fetch("/api/weight");
  return response.json();
};

const Weight = () => {
  const [data] = createResource(fetchWeight);

  return (
    <div>
      <div class="text-xl font-bold p-8">Weight</div>
      <div class="max-w-2xl mx-auto mt-16">
        <div class="p-2 flex flex-col gap-2 left-2 md:left-auto">
          <Show when={data()}>
            {Plot.plot({
              y: {
                grid: true,
                domain: [90, 100],
              },
              marks: [
                Plot.lineY(data()?.weights, {
                  x: (d) => new Date(d.timestamp),
                  y: "weight",
                }),
                Plot.lineY(data()?.smooth_weights, {
                  x: (d) => new Date(d.timestamp),
                  y: "weight",
                  stroke: "red",
                  opacity: 0.3,
                }),
              ],
            })}
          </Show>
        </div>
      </div>
    </div>
  );
};

export default Weight;
