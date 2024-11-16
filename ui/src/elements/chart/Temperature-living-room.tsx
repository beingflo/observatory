import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../../components/Chart";
import { Card } from "../../components/Card";
import { useRange } from "../../components/RangeProvider";
import { getDate } from "../../components/DateRangeSelector";

const fetchData = async (from: string) => {
  const response = await fetch(
    `/api/data?bucket=co2-sensor-living-room&sample=1000&from=${from}`
  );
  return response.json();
};

export const TemperatureLivingRoom = () => {
  const [{ from }] = useRange();
  const [data, { refetch }] = createResource(from, () =>
    fetchData(getDate(from()))
  );

  setTimeout(() => refetch(), 30000);

  return (
    <Card title="Temperature living room">
      <Chart
        id="temperature-living-room"
        loading={data.loading}
        plot={{
          y: {
            grid: true,
            label: "Temperature [°C]",
          },
          x: {
            type: "time",
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
