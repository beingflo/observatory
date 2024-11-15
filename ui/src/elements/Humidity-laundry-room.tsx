import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../components/Chart";
import { Card } from "../components/Card";

const fetchData = async (hours: number) => {
  const now = new Date();
  now.setHours(now.getHours() - hours);

  const response = await fetch(
    `/api/data?bucket=humidity-laundry-room&from=${now.toISOString()}`
  );
  return response.json();
};

export const HumidityLaundryRoom = () => {
  const [data, { refetch }] = createResource(() => fetchData(48));

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="Humidity laundry room">
      <Chart
        id="humdity-laundry-room"
        loading={!data()}
        plot={{
          y: {
            grid: true,
            label: "Humidity [%]",
          },
          x: {
            type: "time",
          },
          marks: [
            Plot.lineY(data(), {
              x: (d) => new Date(d.timestamp),
              y: (d) => d.payload.humidity,
            }),
          ],
        }}
      />
    </Card>
  );
};
