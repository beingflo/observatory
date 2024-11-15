import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../components/Chart";

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
  const [data] = createResource(() => fetchHomeData(6));

  return (
    <div class="p-8">
      <div class="text-xl font-bold pb-8">Home</div>
      <div class="w-full">
        <Chart
          id="co2"
          loading={!data()}
          plot={{
            y: {
              grid: true,
            },
            marks: [
              Plot.lineY(data()?.data, {
                x: (d) => new Date(d.timestamp),
                y: "co2",
              }),
            ],
          }}
        />
      </div>
    </div>
  );
};

export default HomeData;
