import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../components/Chart";
import { Card } from "../components/Card";

const fetchData = async (hours: number) => {
  const now = new Date();
  now.setHours(now.getHours() - hours);

  const response = await fetch(
    `/api/data?bucket=co2-sensor-living-room&from=${now.toISOString()}`
  );
  return response.json();
};

export const CO2LivingRoom = () => {
  const [data, { refetch }] = createResource(() => fetchData(12));

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
            Plot.lineY(data(), {
              x: (d) => new Date(d.timestamp),
              y: (d) => d.payload.co2,
            }),
          ],
        }}
      />
    </Card>
  );
};
