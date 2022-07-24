import './index.css';
import { Buffer } from 'buffer';
import React from 'react';
import ReactDOM from 'react-dom/client';
import Router from './router';
import { NearEnvironment } from 'react-near';
import { ProviderNear } from './hooks/near';
import { ChakraProvider, extendTheme } from '@chakra-ui/react';
import 'slick-carousel/slick/slick.css';
import 'slick-carousel/slick/slick-theme.css';
import { Toaster } from 'react-hot-toast';

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
        <Toaster
          position="bottom-left"
          toastOptions={{
            className: 'h-[66px] bg-[#FF6F00] font-titan color-white text-white rounded-[12px]',
          }}
        />
      </ProviderNear>
    </ChakraProvider>
  </React.StrictMode>
);
