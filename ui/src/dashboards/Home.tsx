import { Dashboard } from "../components/Dashboard";
import { BarometricLatestReadingNook } from "../elements/Barometric-latest-reading-nook";
import { BrightnessLatestReadingNook } from "../elements/Brightness-latest-reading-nook";
import { CO2LatestLivingRoom } from "../elements/CO2-latest-living-room";
import { CO2LivingRoom } from "../elements/CO2-living-room";
import { HumidityLatestLaundryRoom } from "../elements/Humidity-latest-laundry-room";
import { HumidityLatestLivingRoom } from "../elements/Humidity-latest-living-room";
import { TemperatureLatestLaundryRoom } from "../elements/Temperature-latest-laundry-room";
import { TemperatureLatestLivingRoom } from "../elements/Temperature-latest-living-room";

export const Home = () => {
  return (
    <Dashboard title="Home">
      <div class="flex flex-col gap-8">
        <CO2LivingRoom />
        <div class="flex flex-row gap-8">
          <div class="w-48">
            <CO2LatestLivingRoom />
          </div>
          <div class="w-48">
            <HumidityLatestLivingRoom />
          </div>
          <div class="w-48">
            <HumidityLatestLaundryRoom />
          </div>
          <div class="w-48">
            <TemperatureLatestLivingRoom />
          </div>
          <div class="w-48">
            <TemperatureLatestLaundryRoom />
          </div>
          <div class="w-48">
            <BrightnessLatestReadingNook />
          </div>
          <div class="w-48">
            <BarometricLatestReadingNook />
          </div>
        </div>
      </div>
    </Dashboard>
  );
};
