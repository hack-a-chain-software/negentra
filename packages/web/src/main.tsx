import "./index.css";
import { Buffer } from "buffer";
import React from "react";
import ReactDOM from "react-dom/client";
import Router from "./router";
import { NearEnvironment } from "react-near";
import { ProviderNear } from "./hooks/near";
import { ChakraProvider, extendTheme } from '@chakra-ui/react';
import 'slick-carousel/slick/slick.css';
import 'slick-carousel/slick/slick-theme.css';

const theme = extendTheme({
  fonts: {
    body: `'Open Sans', sans-serif`,
  },
});

// TODO: Find a better way to handle this buffer error
window.Buffer = window.Buffer || Buffer;

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ChakraProvider theme={theme}>
      <ProviderNear environment={NearEnvironment.TestNet}>
        <Router />
      </ProviderNear>
    </ChakraProvider>
  </React.StrictMode>
);
