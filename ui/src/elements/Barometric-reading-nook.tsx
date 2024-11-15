import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../components/Chart";
import { Card } from "../components/Card";

const fetchData = async (hours: number) => {
  const now = new Date();
  now.setHours(now.getHours() - hours);

  const response = await fetch(
    `/api/data?bucket=brightness-barometer-living-room&from=${now.toISOString()}`
  );
  return response.json();
};

export const BarometricReadingNook = () => {
  const [data, { refetch }] = createResource(() => fetchData(24));

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="Barometric pressure">
      <Chart
        id="barometric-pressure-reading-nook"
        loading={!data()}
        plot={{
          y: {
            grid: true,
            label: "pressure [hPA]",
          },
          x: {
            type: "time",
          },
          marginLeft: 50,
          marks: [
            Plot.lineY(data(), {
              x: (d) => new Date(d.timestamp),
              y: (d) => d.payload.pressure,
            }),
          ],
        }}
      />
    </Card>
  );
};
