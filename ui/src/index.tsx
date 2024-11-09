/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import Weight from "./dashboards/Weight";
import HomeData from "./dashboards/HomeData";

const root = document.getElementById("root");

render(
  () => (
    <>
      <Router>
        <Route path="/" component={App} />
        <Route path="/weight" component={Weight} />
        <Route path="/home" component={HomeData} />
      </Router>
    </>
  ),
  root!
);
