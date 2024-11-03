/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import Weight from "./dashboards/Weight";

const root = document.getElementById("root");

render(
  () => (
    <>
      <Router>
        <Route path="/" component={App} />
        <Route path="/weight" component={Weight} />
      </Router>
    </>
  ),
  root!
);
