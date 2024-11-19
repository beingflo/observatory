import { Dashboard } from "../components/Dashboard";
import { BarometricReadingNook } from "../elements/chart/Barometric-reading-nook";
import { BrightnessReadingNook } from "../elements/chart/Brightness-reading-nook";
import { CO2LivingRoom } from "../elements/chart/CO2-living-room";
import { HumidityLaundryRoom } from "../elements/chart/Humidity-laundry-room";
import { HumidityLivingRoom } from "../elements/chart/Humidity-living-room";
import { TemperatureLivingRoom } from "../elements/chart/Temperature-living-room";
import { BarometricLatestReadingNook } from "../elements/status/Barometric-latest-reading-nook";
import { BrightnessLatestReadingNook } from "../elements/status/Brightness-latest-reading-nook";
import { CO2LatestLivingRoom } from "../elements/status/CO2-latest-living-room";
import { HumidityLatestLaundryRoom } from "../elements/status/Humidity-latest-laundry-room";
import { HumidityLatestLivingRoom } from "../elements/status/Humidity-latest-living-room";
import { TemperatureLatestLaundryRoom } from "../elements/status/Temperature-latest-laundry-room";
import { TemperatureLatestLivingRoom } from "../elements/status/Temperature-latest-living-room";

export const Home = () => {
  return (
    <Dashboard title="Home">
      <div class="flex flex-col gap-8">
        <div class="grid grid-cols-2 md:grid-cols-4 2xl:grid-cols-8 3xl:grid-cols-12 gap-8">
          <CO2LatestLivingRoom />
          <HumidityLatestLivingRoom />
          <HumidityLatestLaundryRoom />
          <TemperatureLatestLivingRoom />
          <TemperatureLatestLaundryRoom />
          <BrightnessLatestReadingNook />
          <BarometricLatestReadingNook />
        </div>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <CO2LivingRoom />
          <HumidityLivingRoom />
          <HumidityLaundryRoom />
          <TemperatureLivingRoom />
          <BarometricReadingNook />
          <BrightnessReadingNook />
        </div>
      </div>
    </Dashboard>
  );
};
