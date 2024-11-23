import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../../components/Chart";
import { Card } from "../../components/Card";
import { useRange } from "../../components/RangeProvider";
import { getRandomInRange } from "../../components/utils";

const fetchData = async (from: string, to: string) => {
  const response = await fetch(
    `/api/data?bucket=co2-sensor-living-room&sample=1000&from=${from}&to=${to}`
  );
  return response.json();
};

export const CO2LivingRoom = () => {
  const [{ from, to }] = useRange();
  const [data, { refetch }] = createResource(
    () => [from(), to()],
    () => fetchData(from(), to())
  );

  setTimeout(() => {
    refetch();
    setInterval(() => refetch(), 30000);
  }, getRandomInRange(0, 30000));

  return (
    <Card title="CO2 living room">
      <Chart
        id="co2-living-room"
        loading={data.loading}
        plot={{
          y: {
            grid: true,
            label: "CO2 [ppm]",
          },
          x: {
            type: "time",
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
