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

export const TemperatureLivingRoom = () => {
  const [data, { refetch }] = createResource(() => fetchData(24));

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="Temperature living room">
      <Chart
        id="temperature-living-room"
        loading={!data()}
        plot={{
          y: {
            grid: true,
            label: "Temperature [°C]",
          },
          marks: [
            Plot.lineY(data(), {
              x: (d) => new Date(d.timestamp),
              y: (d) => d.payload.temperature,
            }),
          ],
        }}
      />
    </Card>
  );
};
