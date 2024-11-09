import { createResource, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

const fetchHomeData = async () => {
  const response = await fetch("/api/home");
  return response.json();
};

const HomeData = () => {
  const [data] = createResource(fetchHomeData);

  return (
    <div>
      <div class="text-xl font-bold p-8">Home</div>
      <div class="p-12 w-full mx-auto mt-16">
        <div class="p-2 w-full grid grid-cols-3 gap-12">
          <Show when={data()}>
            {Plot.plot({
              y: {
                grid: true,
              },
              marks: [
                Plot.lineY(data()?.data, {
                  x: (d) => new Date(d.timestamp),
                  y: "co2",
                }),
              ],
            })}
          </Show>
          <Show when={data()}>
            {Plot.plot({
              y: {
                grid: true,
              },
              marks: [
                Plot.lineY(data()?.data, {
                  x: (d) => new Date(d.timestamp),
                  y: "temperature",
                }),
              ],
            })}
          </Show>
          <Show when={data()}>
            {Plot.plot({
              y: {
                grid: true,
              },
              marks: [
                Plot.lineY(data()?.data, {
                  x: (d) => new Date(d.timestamp),
                  y: "humidity",
                }),
              ],
            })}
          </Show>
        </div>
      </div>
    </div>
  );
};

export default HomeData;
