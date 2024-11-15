import { createResource } from "solid-js";
import { Status } from "../components/Status";

const getData = async () => {
  const response = await fetch(
    `/api/data?bucket=humidity-laundry-room&limit=1`
  );
  const json = await response.json();
  return json?.[0]?.payload?.humidity;
};

export const HumidityLatestLaundryRoom = () => {
  const [data, { refetch }] = createResource(() => getData(), {
    initialValue: 0,
  });

  setTimeout(() => refetch(), 30000);

  return (
    <Status title="Current humidity laundry room" content={`${data()} %`} />
  );
};
