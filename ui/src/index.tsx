/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import Weight from "./dashboards/Weight";
import { Home } from "./dashboards/Home";
import { RangeProvider } from "./components/RangeProvider";

const root = document.getElementById("root");

render(
  () => (
    <RangeProvider from="1d">
      <Router>
        <Route path="/" component={App} />
        <Route path="/weight" component={Weight} />
        <Route path="/home" component={Home} />
      </Router>
    </RangeProvider>
  ),
  root!
);
