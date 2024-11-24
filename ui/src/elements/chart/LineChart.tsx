import { createResource } from "solid-js";
import * as Plot from "@observablehq/plot";
import { Chart } from "../../components/Chart";
import { Card } from "../../components/Card";
import { useRange } from "../../components/RangeProvider";
import { getRandomInRange } from "../../components/utils";

export type LineChartProps = {
  bucket: string;
  title: string;
  yLabel: string;
  yData: (d: object) => number;
};

export const LineChart = (props: LineChartProps) => {
  const [{ from, to }] = useRange();

  const fetchData = async (from: string, to: string) => {
    const response = await fetch(
      `/api/data?bucket=${props.bucket}&sample=1000&from=${from}&to=${to}`
    );
    return response.json();
  };

  const [data, { refetch }] = createResource(
    () => [from(), to()],
    () => fetchData(from(), to())
  );

  setTimeout(() => {
    refetch();
    setInterval(() => refetch(), 30000);
  }, getRandomInRange(0, 30000));

  return (
    <Card title={props.title}>
      <Chart
        id={props.title}
        loading={!data()}
        plot={{
          y: {
            grid: true,
            label: props.yLabel,
          },
          x: {
            type: "time",
          },
          marks: [
            data() &&
              Plot.lineY(data(), {
                x: (d) => new Date(d.timestamp),
                y: props.yData,
              }),
          ],
        }}
      />
    </Card>
  );
};
