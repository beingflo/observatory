import { createResource, Show } from "solid-js";
import * as Plot from "@observablehq/plot";

const fetchHomeData = async (hours?: number) => {
  if (hours) {
    const now = new Date();
    now.setHours(now.getHours() - hours);
    const response = await fetch(`/api/home?from=${now.toISOString()}`);
    return response.json();
  } else {
    const response = await fetch(`/api/home`);
    return response.json();
  }
};

const HomeData = () => {
  const [data, { refetch }] = createResource(() => fetchHomeData(6));
  const [dataFull, { refetch: refetchFull }] = createResource(() =>
    fetchHomeData()
  );

  setInterval(() => refetch(), 30000);
  setInterval(() => refetchFull(), 30000);

  return (
    <div class="p-8">
      <div class="text-xl font-bold pb-8">Home</div>
      <div class="w-full mx-auto">
        <div class="p-2 w-full grid grid-cols-1 lg:grid-cols-3 gap-12">
          <Show when={data()}>
            {Plot.plot({
              y: {
                grid: true,
              },
              width: screen.availWidth / 3,
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
              width: screen.availWidth / 3,
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
              width: screen.availWidth / 3,
              marks: [
                Plot.lineY(data()?.data, {
                  x: (d) => new Date(d.timestamp),
                  y: "humidity",
                }),
              ],
            })}
          </Show>
        </div>
        <div class="w-full">
          <Show when={dataFull()}>
            {Plot.plot({
              y: {
                grid: true,
              },
              width: screen.availWidth,
              marks: [
                Plot.lineY(dataFull()?.data, {
                  x: (d) => new Date(d.timestamp),
                  y: "co2",
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
