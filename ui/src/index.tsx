/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import Weight from "./dashboards/Weight";
import { Home } from "./dashboards/Home";
import { RangeProvider } from "./components/RangeProvider";
import { Location } from "./dashboards/Location";
import { Heuried } from "./dashboards/Heuried";

const root = document.getElementById("root");

render(
  () => (
    <RangeProvider fromOption="6h">
      <Router>
        <Route path="/" component={App} />
        <Route path="/home" component={Home} />
        <Route path="/location" component={Location} />
        <Route path="/weight" component={Weight} />
        <Route path="/heuried" component={Heuried} />
      </Router>
    </RangeProvider>
  ),
  root!
);
