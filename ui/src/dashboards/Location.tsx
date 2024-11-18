import { createMemo, createResource, createSignal } from "solid-js";
import { Dashboard } from "../components/Dashboard";
import MapGL, { Layer, Source, Viewport } from "solid-map-gl";
import { useRange } from "../components/RangeProvider";
import { getDate } from "../components/DateRangeSelector";

const fetchData = async (from?: string) => {
  const response = await fetch(`/api/gps/location_florian?from=${from}`);
  return response.json();
};

export const Location = () => {
  const [{ from }] = useRange();
  const [viewport, setViewport] = createSignal({
    center: [8.53, 47.37],
    zoom: 13,
  } as Viewport);

  const [data] = createResource(from, () => fetchData(getDate(from())));

  const geoJson = createMemo(() => ({
    type: "Feature",
    geometry: {
      type: "LineString",
      coordinates: data()?.map((d: any) => [d.longitude, d.latitude]),
    },
  }));

  return (
    <Dashboard title="Location">
      <div class="p-0">
        <MapGL
          options={{
            accessToken: import.meta.env.VITE_MAPBOX_ACCESS_TOKEN,
            style: "mb:outdoor",
          }}
          viewport={viewport()}
          onViewportChange={(evt: Viewport) => setViewport(evt)}
        >
          <Source
            source={{
              type: "geojson",
              data: geoJson(),
            }}
          >
            <Layer
              style={{
                type: "line",
                paint: {
                  "line-color": "black",
                  "line-width": 2,
                },
              }}
            />
          </Source>
        </MapGL>
      </div>
    </Dashboard>
  );
};
