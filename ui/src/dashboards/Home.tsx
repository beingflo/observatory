import { Dashboard } from "../components/Dashboard";
import { LineChart } from "../elements/chart/LineChart";
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
      <div class="flex flex-col gap-4">
        <div class="grid grid-cols-2 md:grid-cols-4 2xl:grid-cols-8 3xl:grid-cols-12 gap-2 md:gap-4">
          <CO2LatestLivingRoom />
          <HumidityLatestLivingRoom />
          <HumidityLatestLaundryRoom />
          <TemperatureLatestLivingRoom />
          <TemperatureLatestLaundryRoom />
          <BrightnessLatestReadingNook />
          <BarometricLatestReadingNook />
        </div>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-2 md:gap-4">
          <LineChart
            bucket="co2-sensor-living-room"
            yData={(d: any) => d.payload.co2}
            yLabel="CO2 [ppm]"
            title="CO2 living room"
          />
          <LineChart
            bucket="co2-sensor-living-room"
            yData={(d: any) => d.payload.temperature}
            yLabel="Temperature [°C]"
            title="Temperature living room"
          />
          <LineChart
            bucket="co2-sensor-living-room"
            yData={(d: any) => d.payload.humidity}
            yLabel="Humidity [%]"
            title="Humidity living room"
          />
          <LineChart
            bucket="humidity-laundry-room"
            yData={(d: any) => d.payload.humidity}
            yLabel="Humidity [%]"
            title="Humidity laundry room"
          />
          <LineChart
            bucket="brightness-barometer-living-room"
            yData={(d: any) => d.payload.pressure}
            yLabel="pressure [hPA]"
            title="Barometric pressure"
          />
          <LineChart
            bucket="brightness-barometer-living-room"
            yData={(d: any) => d.payload.lux}
            yLabel="Brightness [lux]"
            title="Brightness kitchen"
          />
        </div>
      </div>
    </Dashboard>
  );
};
