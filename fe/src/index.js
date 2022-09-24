import React from "react";
import { createRoot } from "react-dom/client";
import { initContract } from "../near-api";

// scroll bar
import "simplebar/src/simplebar.css";

import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { HelmetProvider } from "react-helmet-async";

//
import { SnackbarProvider } from "notistack";
import { Provider } from "react-redux";
import App from "./App";
import * as serviceWorker from "./serviceWorker";
import reportWebVitals from "./reportWebVitals";

import { store } from "./redux/store";

const reactRoot = createRoot(document.querySelector("#root"));

window.nearInitPromise = initContract()
  .then(() => {
    reactRoot.render(
      <HelmetProvider>
        <BrowserRouter>
          <SnackbarProvider>
            <Provider store={store}>
              <App />
            </Provider>
          </SnackbarProvider>
        </BrowserRouter>
      </HelmetProvider>
    );
  })
  .catch((e) => {
    reactRoot.render(
      <div style={{ color: "red" }}>
        Error: <code>{e.message}</code>
      </div>
    );
    console.error(e);
  });
