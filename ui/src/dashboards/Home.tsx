import { Dashboard } from "../components/Dashboard";
import { CO2Latest } from "../elements/CO2-latest";
import { CO2LivingRoom } from "../elements/CO2-living-room";

export const Home = () => {
  return (
    <Dashboard title="Home">
      <div class="flex flex-col gap-8">
        <CO2LivingRoom />
        <div class="w-48">
          <CO2Latest />
        </div>
      </div>
    </Dashboard>
  );
};
