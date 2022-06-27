import "./index.css";
import { Buffer } from "buffer";
import React from "react";
import ReactDOM from "react-dom/client";
import Router from "./router";
import { NearEnvironment } from "react-near";
import { ProviderNear } from "./hooks/near";
import { ChakraProvider } from '@chakra-ui/react';

// TODO: Find a better way to handle this buffer error
window.Buffer = window.Buffer || Buffer;

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ChakraProvider>
      <ProviderNear environment={NearEnvironment.TestNet}>
        <Router />
      </ProviderNear>
    </ChakraProvider>
  </React.StrictMode>
);
