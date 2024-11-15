/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import Weight from "./dashboards/Weight";
import { Home } from "./dashboards/Home";
import { RangeProvider } from "./components/RangeProvider";

const root = document.getElementById("root");

const now = new Date();
now.setHours(now.getHours() - 24);

render(
  () => (
    <RangeProvider from={now.toISOString()}>
      <Router>
        <Route path="/" component={App} />
        <Route path="/weight" component={Weight} />
        <Route path="/home" component={Home} />
      </Router>
    </RangeProvider>
  ),
  root!
);
