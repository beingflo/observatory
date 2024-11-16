import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../../components/Chart";
import { Card } from "../../components/Card";
import { useRange } from "../../components/RangeProvider";
import { getDate } from "../../components/DateRangeSelector";
import { getRandomInRange } from "../../components/utils";

const fetchData = async (from: string) => {
  const response = await fetch(
    `/api/data?bucket=brightness-barometer-living-room&sample=1000&from=${from}`
  );
  return response.json();
};

export const BarometricReadingNook = () => {
  const [{ from }] = useRange();
  const [data, { refetch }] = createResource(from, () =>
    fetchData(getDate(from()))
  );

  setTimeout(() => {
    refetch();
    setInterval(() => refetch(), 30000);
  }, getRandomInRange(0, 30000));

  return (
    <Card title="Barometric pressure">
      <Chart
        id="barometric-pressure-reading-nook"
        loading={data.loading}
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