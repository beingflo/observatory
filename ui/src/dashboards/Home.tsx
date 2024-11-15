import { Dashboard } from "../components/Dashboard";
import { BarometricLatestReadingNook } from "../elements/Barometric-latest-reading-nook";
import { BrightnessLatestReadingNook } from "../elements/Brightness-latest-reading-nook";
import { CO2LatestLivingRoom } from "../elements/CO2-latest-living-room";
import { CO2LivingRoom } from "../elements/CO2-living-room";
import { HumidityLatestLaundryRoom } from "../elements/Humidity-latest-laundry-room";
import { HumidityLatestLivingRoom } from "../elements/Humidity-latest-living-room";
import { HumidityLaundryRoom } from "../elements/Humidity-laundry-room";
import { HumidityLivingRoom } from "../elements/Humidity-living-room";
import { TemperatureLatestLaundryRoom } from "../elements/Temperature-latest-laundry-room";
import { TemperatureLatestLivingRoom } from "../elements/Temperature-latest-living-room";

export const Home = () => {
  return (
    <Dashboard title="Home">
      <div class="flex flex-col gap-8">
        <div class="grid grid-cols-4 2xl:grid-cols-12 gap-8">
          <CO2LatestLivingRoom />
          <HumidityLatestLivingRoom />
          <HumidityLatestLaundryRoom />
          <TemperatureLatestLivingRoom />
          <TemperatureLatestLaundryRoom />
          <BrightnessLatestReadingNook />
          <BarometricLatestReadingNook />
        </div>
        <div class="grid grid-cols-2 gap-8">
          <CO2LivingRoom />
          <HumidityLivingRoom />
        </div>
        <HumidityLaundryRoom />
      </div>
    </Dashboard>
  );
};
