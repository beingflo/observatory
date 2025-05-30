import { Dashboard } from "../components/Dashboard";
import { LineChart } from "../elements/chart/LineChart";

export const Heuried = () => {
  return (
    <Dashboard title="Heuried">
      <div class="grid grid-cols-1 lg:grid-cols-1 gap-2 md:gap-4">
        <LineChart
          bucket="heuried-visitors"
          yData={(d: any) => d.payload}
          yLabel="Visitors [#]"
          title="Current visitors"
        />
      </div>
    </Dashboard>
  );
};
