import { createResource } from "solid-js";
import { Status } from "../../components/Status";
import { getRandomInRange } from "../../components/utils";

const getData = async () => {
  const response = await fetch(
    `/api/data?bucket=brightness-barometer-living-room&limit=1`
  );
  const json = await response.json();
  return json?.[0]?.payload?.pressure;
};

export const BarometricLatestReadingNook = () => {
  const [data, { refetch }] = createResource(() => getData(), {
    initialValue: 0,
  });

  setTimeout(() => {
    refetch();
    setInterval(() => refetch(), 30000);
  }, getRandomInRange(0, 30000));

  return (
    <Status title="Current barometric pressure" content={`${data()} hPa`} />
  );
};
