import { createResource } from "solid-js";
import { Status } from "../../components/Status";
import { getRandomInRange } from "../../components/utils";

const getData = async () => {
  const response = await fetch(
    `/api/data?bucket=co2-sensor-living-room&limit=1`
  );
  const json = await response.json();
  return json?.[0]?.payload?.temperature;
};

export const TemperatureLatestLivingRoom = () => {
  const [data, { refetch }] = createResource(() => getData(), {
    initialValue: 0,
  });

  setTimeout(() => {
    refetch();
    setInterval(() => refetch(), 30000);
  }, getRandomInRange(0, 30000));

  return <Status title="Current temperature" content={`${data()} °C`} />;
};
