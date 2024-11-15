import { Card } from "../components/Card";
import { Dashboard } from "../components/Dashboard";
import { CO2LivingRoom } from "../elements/CO2-living-room";

export const Home = () => {
  return (
    <Dashboard title="Home">
      <Card title="CO2 living room">
        <CO2LivingRoom />
      </Card>
    </Dashboard>
  );
};
