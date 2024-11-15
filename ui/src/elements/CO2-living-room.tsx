import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../components/Chart";
import { Card } from "../components/Card";

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

export const CO2LivingRoom = () => {
  const [data, { refetch }] = createResource(() => fetchHomeData(6));

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="CO2 living room">
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
    </Card>
  );
};
