import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../../components/Chart";
import { Card } from "../../components/Card";
import { useRange } from "../../components/RangeProvider";

const fetchData = async (from: string) => {
  const response = await fetch(
    `/api/data?bucket=humidity-laundry-room&from=${from}`
  );
  return response.json();
};

export const HumidityLaundryRoom = () => {
  const [{ from }] = useRange();
  const [data, { refetch }] = createResource(from, () => fetchData(from()));

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="Humidity laundry room">
      <Chart
        id="humdity-laundry-room"
        loading={data.loading}
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
